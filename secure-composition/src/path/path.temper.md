# Secure Path Composition

Provides ways to securely and reliably compose file paths.

You can join two paths.

    test("simple join") {
      let dir = "foo";
      let file = "bar.ext";
      let ps = path"${dir}/${file}".posixString;
      assert(path"${dir}/${file}".posixString == "foo/bar.ext");
    }

Some subtleties make it difficult to reliably produce paths using
simple string concatenation.  First, lets talk about some goals and hazards.

## Goals

**Goal**: Joining two relative paths produces a relative path.

    test("accidentally absolute hazard") {
      let dir = "";  // empty string
      let file = "file.ext";

      // With simple string concatenation, joining a path on '/'
      // leads to a path that is not relative as intended, but
      // is accidentally absolute.
      assert(    "${dir}/${file}" == "/file.ext");

      // But the path tag does better.
      assert(path"${dir}/${file}".posixString == "file.ext");
    }

**Goal**: path composition does not involve file-system checks, it is purely
lexical.

    test("files are not special") {
      let base = "/etc/profile";  // A file on every UNIX
      let rel = "./foo";

      assert(path"${base}/${rel}".posixString == "/etc/profile/foo");

      // Starting with a `./` prevents base from being treated
      // as absolute, so is a reliable way to force relative URL creation.
      assert(path"./${base}/${rel}".posixString == "etc/profile/foo");
    }

Resolving a relative path does not depend on whether the base path
specifies a file or directory so does not requiring `stat`ing or any
other operation that can change based on intermittent file system
failures, and involve unpredictable delays, e.g. on a network file
system mount.

**Goal**: File names can be composed of parts.

    test("strings without / can be path affixes") {
      let ext = ".png";
      let num = "1234";
      let prefix = "unicorn";

      assert(path"sparklies/${prefix}#${num}${ext}".posixString == "sparklies/unicorn#1234.png");
    }

**Goal**: Empty strings specify no path segments

    test("empty strings specify no path segments") {
      let nothing = "";
      assert(path"foo/${ nothing }/bar".posixString == "foo/bar");
    }

**Goal**: Paths can opt-into parent traversal, but by default,
interpolations are blocked navigating upwards so are safe from
[CWE-22], path traversal attacks.

    test("cwe-22") {
      let attack = "../../../../etc/passwd";
      assert(path"session-files/1234/uploads/${attack}".posixString == "/dev/null/zz_Temper_zz");

      let ok = "a/b/../c"; // Internal .. is ok
      assert(path"session-files/1234/uploads/${ok}".posixString == "session-files/1234/uploads/a/c");
    }

    test(".. in trusted parts ok") {
      let abc = "a/b/c";
      assert(path"${abc}/..".posixString == "a/b");
      //                 ^^ trusted
    }

    test("explicit upwards traversal allowed") {
      let upPath = "../../foo";
      let shortUpPath = "../foo";

      // /<, '<' after a slash, allows for upwards traversal
      assert(path"dir/<${upPath}".posixString == "../foo");
      // </, '<' before the slash, blocks upwards traversal
      assert(path"base</subdir/<${shortUpPath}".posixString == "base/foo");
      //              ^^      ^^
      assert(path"base</subdir/<${upPath}".posixString == "/dev/null/zz_Temper_zz");
    }

    test("can escape meta characters") {
      // '<' has a special meaning above in two contexts.
      // But you can use it in a path.

      let pathFragment = "truths/1<2";
      assert(path"predicates/${pathFragment}".posixString == "predicates/truths/1<2");
      // You can also escape '<' and '\' inside fixed parts.
      assert(path"operators/\</${pathFragment}".posixString == "operators/</truths/1<2");
      //           Escape   ^^
    }

The rule here is that `/<` allows following interpolations to move
backwards through slashes.

But `/<` is a slash reinforced by a catcher. Regardless of what comes after,
it blocks upwards traversal by interpolations.

**Goal**: An path string on the right is never an absolute path.
It can contribute a `/` which breaks segments, but that does not make it
absolute.

    test("interpolations are not absolute") {
      let looksAbsolute = "/foo";

      assert(path"bar/${looksAbsolute}/baz".posixString == "bar/foo/baz");
      assert(path"bar${looksAbsolute}/baz".posixString == "bar/foo/baz");
      //             ^ No / before looksAbsolute, but one here^

      // Changing whether we allow upwards traversal, doesn't affect it.
      assert(path"bar/<${looksAbsolute}/baz".posixString == "bar/foo/baz");
    }

**Goal**: You can specify Windows paths, just using consistent
POSIX / URL path separators internally.

    test("windows path support") {
      let absDir = "C:/";
      let relDir = "Users/ItsMe/AppData/Microsoft Games/Solitaire";

      assert(path"${absDir}/${relDir}".winString ==
             raw"C:\Users\ItsMe\AppData\Microsoft Games\Solitaire");
    }

**Goal**: Path objects compose into path objects.

    test("composition") {
      // Same test-case as at top, but `dir` and `file` use `path`.
      let dir = path"foo";
      let file = path"bar.ext";
      assert(path"${dir}/${file}".posixString == "foo/bar.ext");
    }

**Goal**: From a *FilePath* value, we can get any of the following string forms:

- A POSIX style path with `/` separating file segments.
- A Windows style path with `\` separating file segments.
- A `file:` URL with URL meta-characters like `#` escaped using percent-encoding.

Any file path value can be asked for any of those.
They are not internally one or another.

    test("multiple representations") {
      let dir = "C:/Users/Alice";
      let relFile = "/dir/need-escaping # ? ";

      let p = path"${dir}/${relFile}";

      assert(p.posixString   ==         "/c/Users/Alice/dir/need-escaping # ? ");
      assert(p.winString     ==      raw"C:\Users\Alice\dir\need-escaping # ? ");
      assert(p.fileUrlString == "file:///c|/Users/Alice/dir/need-escaping%20%23%20%3f%20");
    }

Windows paths presented as POSIX strings do not have a WSL `/mnt` or
[Cygwin `/cygdrive` prefix](https://cygwin.com/cygwin-ug-net/using.html#using-pathnames).

**Goal**: Error paths are identifiable.

You can ask a path if there was an error during composition.
If you forget, that path is still innocuous.

    test("error paths") {
      let devNull = "/dev/null";
      let oddRelPath = "zz_Temper_zz"; // Odd choice of name
      let evilRelPath = "../../../etc/passwd";

      assert(!path"${devNull}/${oddRelPath}".isBad);
      assert( path"${devNull}/${evilRelPath}".isBad);
    }

## The FilePath type

FilePath represents a path on the file system.
It's an abstract grouping of parts, not purely textual, so you
can get a POSIX string, a Windows path string, or a file URL.

    export class FilePath(

winDrive is null, or the ASCII letter for the windows drive.
For example, "C:" is represented as 99, ASCII lower-case 'c'.

      public winDrive: Int32?,

parts is a list of strings representing the elements of the path.
It may have the empty string as the first element indicating an
absolute path; nothing before the first separator.

      public parts: List<String>,

isBad is true when the file path was malformed.
If so, its POSIX path will be `/dev/null/zz_Temper_zz`.
(It is hoped that by using *zz\_Temper\_zz*, developers who
encounter this in bug reports will find their way to this documentation)

      public isBad: Boolean,
    ) {

isAbsolute is true when the path is absolute, either because
there is a windows drive or because the first character in a textual
representation is a separator indicating it starts at the file-system
root.

      public get isAbsolute(): Boolean {
        winDrive != null || (!parts.isEmpty && parts[0] == "")
      }

posixString is a POSIX style representation of the path that uses
`/` to separate components.

      public get posixString(): String {
        let sb = new StringBuilder();
        var separated = false;
        let winDrive = this.winDrive;
        if (winDrive != null) {
          sb.append("/");
          sb.appendCodePoint(winDrive | 32) orelse panic();
          separated = true;
        }
        let parts: List<String> = this.parts;
        for (let part of parts) {
          if (separated) { sb.append("/"); }
          sb.append(part);
          separated = true;
        }
        sb.toString()
      }

winString is a windows style representation of the path that uses `\`
to separate components and which renders windows drives in upper case,
with a colon: `C:`.

      public get winString(): String {
        let sb = new StringBuilder();
        var separated = false;
        let winDrive = this.winDrive;
        if (winDrive != null) {
          sb.appendCodePoint(winDrive & 0xFFFF_FFDF) orelse panic();
          sb.append(":");
          separated = true;
        }
        for (let part of parts) {
          if (separated) { sb.append("\\"); }
          sb.append(part);
          separated = true;
        }
        sb.toString()
      }

fileUrlString is a URL representation of the path.  It starts with
`file:` and if the path is absolute, follows that with `///`.
Windows drives are represented as the drive letter followed by pipe (`|`).
URL metacharacters are escaped, so the URL will never have a query part or
fragment when parsed as a hierarchical URL.

      public get fileUrlString(): String {
        let sb = new StringBuilder();
        var separated = false;
        sb.append("file:");
        let winDrive = this.winDrive;
        if (winDrive != null) {
          // Windows drive file part
          sb.append("///");
          sb.appendCodePoint(winDrive | 32) orelse panic();
          sb.append("|");
          separated = true;
        } else if (!parts.isEmpty && parts[0] == "") {
          // Absolute file part
          sb.append("//");
          separated = true;
        }
        for (let part of parts) {
          if (separated) { sb.append("/"); }

          let end = part.end;
          var i = String.begin;
          var upTo = String.begin;
          while (i < end) { // Do minimal percent-encoding
            let cp = part[i];
            let next = part.next(i);

            let esc = when (cp) {
              char'#' -> true; // URL fragment metacharacter
              char'%' -> true; // URL escape metacharacter
              char':' -> true; // URL scheme and authority metacharacter
              char'?' -> true; // URL query metacharacter
              char'/' -> true; // URL path metacharacter
              else -> cp <= 32 // Space and ASCII control characters
              // User agents tend to strip ASCII spaces from ends of URLs.
            }

            if (esc) {
              sb.appendBetween(part, upTo, i);
              upTo = next;
              percentEscapeOctetTo(cp, sb);
            }
            i = next;
          }
          sb.appendBetween(part, upTo, end);

          separated = true;
        }
        sb.toString()
      }
    }

    let driveLetter(s: String): Int32? {
      let end = s.end;
      if (end != String.begin) {
        let back1 = s.prev(end);
        if (back1 != String.begin && s[back1] == char':') {
          let back2 = s.prev(back1);
          let driveLetter = s[back2] | 32;
          if (back2 == String.begin && char'a' <= driveLetter && driveLetter <= char'z') {
            return driveLetter
          }
        }
      }
      null
    }

    let badFilePath: FilePath =
      new FilePath(null, ["", "dev", "null", "zz_Temper_zz"], true);

## The path tag

path is a string expression tag for building [FilePath]s.
It allows composing paths from string parts and other *FilePath*s.

Syntax:

- `/` is the only file separator usable in path string expressions.
- When a `<` follows a file separator, `/<`, it explicitly allows the segment
  following it to use `..` to traverse to an ancestor.
- A `<` before a file separator (`</`) blocks further ancestor traversal allowed
  by `/<`.

By default, ancestor traversal is blocked to prevent [CWE-22] vulnerabilities
so the `<`s around `/`s are necessary to enable or enable it in a limited manner.

Paths are auto-normalized so `.` and `..` file segments are simplified out
where possible, but `..` at the beginning are preserved so `path"../../../${x}"`
is a relative path with three ancestor traversals.

Reverse solidus (`\\`) is not a path separator.  Instead, it can be used to escape
`<` characters that should not be treated as navigation directives.
If you need a windows path, just use [FilePath::winString].

See test cases above for usage examples.

    export let path = FilePathBuilder;

FilePathBuilder allows composing file paths from parts as described above.

    class FilePathBuilder {
      /** True if the current name was preceded by a `/<` marker allowing up navigation. */
      private var currentMayUp: Boolean = false;
      /**
       * The index in parts of the part which disallows up navigation.
       * This is the number of parts on the list when a `</` marker was encountered.
       */
      private var upLimit: Int32 = 0;
      /**
       * The first part processed is special because any leading `/` is
       * an absolute marker, not a separator.
       */
      private var isAtStart: Boolean = true;
      /** The number of `..` segments implicitly before parts */
      private var dotDotSegments: Int32 = 0;
      /** File path parts: directory or file names. */
      private parts: ListBuilder<String> = new ListBuilder();
      /** A name that needs to end up on parts when complete. */
      private currentPart: ListBuilder<String> = new ListBuilder();
      /**
       * True when every part in currentPart comes from a safe source.
       * A safe source is one specified in literal text via appendSafe,
       * and not in a regular append interpolation.
       */
      private var currentIsSafe: Boolean = false;
      /** True if we've encountered a problem or policy violation. */
      private var isBad: Boolean = false;

      private reset(): Void {
        this.currentMayUp = false;
        this.upLimit = 0;
        this.isAtStart = true;
        this.dotDotSegments = 0;
        this.parts.clear();
        this.currentPart.clear();
        // Intentionally does not reset isBad.
      }

      public appendSafe(safePath: String): Void {
        appendInternal(safePath, true);
      }

      @overload("append")
      public appendString(pathStringFragment: String): Void {
        appendInternal(pathStringFragment, false);
      }

      private addCurrentPart(fragment: String, isSafe: Boolean): Void {
        // currentIsSafe is true when all its fragments had the isSafe flag.
        if (!isSafe || currentPart.isEmpty) { this.currentIsSafe = isSafe; }
        currentPart.add(fragment);
      }

      private flushCurrentPart(localStart: Int32): Void {
        if (!currentPart.isEmpty) {
          let isSafe = currentIsSafe;
          let str = currentPart.join("") { s => s };
          when (str) {
            "." -> void;
            ".." -> if (parts.isEmpty) {
              dotDotSegments += 1;
              // It's illegal to use this to resolve a file
              // without resolving it like path"${pwd}/<${startsWithDottyDots}"
            } else if (
              isSafe || // `..` appears in privileged text
              parts.length > localStart || // removing segment added by same string
              currentMayUp && parts.length > upLimit // Explicitly allowed
            ) {
              parts.removeLast();
            } else {
              isBad = true;
            }
            else -> do {
              parts.add(str);
            }
          }
          currentPart.clear();
          // It's up to any processor of an explicit '/' in a safe part to
          // reset currentMayUp.
        }
      }

      /**
       * Called to find segments and partial segments and process them onto
       * currentPart and parts.
       *
       * isSafePart controls whether `..` segments are treated as privileged.
       */
      private appendInternal(pathFragment: String, isSafePart: Boolean): Void {
        let decodeMeta = isSafePart;

        var idx = String.begin;
        let end = pathFragment.end;

        let localStart = parts.length;

        // Break chunks on '/' updating the parts list.
        var beforeFragment = idx;
        while (idx < end) {
          var scan = idx;
          // Scan, looking for `</`, `/`, and `/<`, and `\<`.
          var hasLtBefore = false; // '<' before any '/'.
          var hasSlash = false; // Saw a '/' so need to break a part.
          var hasLtAfter = false; // Saw "/<"
          // </< is a bit self defeating but is semantically allowable.

          let cp = pathFragment[scan];
          if (decodeMeta && char'<' == cp) {
            let afterLp = pathFragment.next(idx);
            if (afterLp < end && char'/' == pathFragment[afterLp]) {
              hasLtBefore = true;
              hasSlash = true;
              scan = afterLp;
            }
          } else if (char'/' == cp) {
            hasSlash = true;
          }
          if (hasSlash) {
            // Empty path segments are ignorable.
            // "a//b" is semantically equivalent to "a/b"
            while (true) {
              let afterSlash = pathFragment.next(scan);
              if (afterSlash < end && char'/' == pathFragment[afterSlash]) {
                scan = afterSlash;
              } else {
                break;
              }
            }

            if (decodeMeta) {
              let afterSlash = pathFragment.next(scan);
              if (afterSlash < end && char'<' == pathFragment[afterSlash]) {
                hasLtAfter = true;
                scan = afterSlash;
              }
            }

            let chunkBefore = pathFragment.slice(beforeFragment, idx);
            if (!chunkBefore.isEmpty ||
                // We allow an empty chunk at the beginning to distinguish
                // absolute paths.
                isAtStart) {
              addCurrentPart(chunkBefore, isSafePart);
            }
            flushCurrentPart(localStart);
            if (hasLtBefore) {
              upLimit = parts.length;
            }
            if (decodeMeta) {
              // If it could have been specified, clear it if not.
              currentMayUp = hasLtAfter;
            }
            beforeFragment = pathFragment.next(scan);
          } else {
            // Don't consume any '<' in the middle of a fragment.
            // Reset scan so the update to idx below doesn't depend on
            // adjustments made to find a slash which ultimately failed.
            scan = idx;
            if (decodeMeta && char'\\' == cp) {
              // any '<' after is literal.
              let afterRevSolidus = pathFragment.next(idx);
              if (afterRevSolidus < end) {
                let chunkBefore = pathFragment.slice(beforeFragment, idx);
                if (!chunkBefore.isEmpty) {
                  addCurrentPart(chunkBefore, isSafePart);
                }
                beforeFragment = afterRevSolidus;
                scan = afterRevSolidus;
              }
            }
          }

          idx = pathFragment.next(scan);
          isAtStart = false;
        }

        // An empty part at the start defangs absolute markers
        isAtStart = false;

        let chunk = pathFragment.slice(beforeFragment, end);
        if (!chunk.isEmpty) {
          addCurrentPart(chunk, isSafePart);
        }
      }

      @overload("append")
      public appendFilePath(path: FilePath): Void {
        let isAbsolute = path.isAbsolute;
        let winDrive = path.winDrive;
        let pathParts = path.parts;

        // If it's an absolute path, just blow away our existing state.
        if (isAbsolute) {
          reset();
        }
        if (path.isBad) { this.isBad = true; }
        let localStart = parts.length;

        // Replay the parts.
        if (winDrive != null) {
          let winDriveBuffer = new StringBuilder();
          winDriveBuffer.appendCodePoint(winDrive & 0xFFFF_FFDF) orelse panic();
          winDriveBuffer.append(":");
          parts.add(winDriveBuffer.toString());
        } else if (isAbsolute) {
          parts.add("");
        }
        for (let part of pathParts) {
          flushCurrentPart(localStart);
          addCurrentPart(part, false);
        }
      }

      public get accumulated(): FilePath {
        flushCurrentPart(-1);
        // Flushing here acts as if there is a '/' at the end which
        // is problematic if the builder is going to be reused.
        // TODO: pick between being destructive or reusable.

        if (isBad) {
          return badFilePath;
        }

        let parts = this.parts;

        var winDrive: Int32? = null;
        let resultParts = new ListBuilder<String>();
        if (dotDotSegments > 0) {
          var nDotDot = dotDotSegments;
          while (nDotDot != 0) {
            resultParts.add("..");
            nDotDot -= 1;
          }
        } else if (!parts.isEmpty) {
          let part0 = parts[0];
          winDrive = driveLetter(part0);
        }

        var nParts = parts.length;
        for (var i = (if (winDrive == null) { 0 } else { 1 });
             i < nParts;
             ++i) {
          resultParts.add(parts[i]);
        }

        new FilePath(winDrive, resultParts.toList(), false)
      }
    }

## Corner cases

    test("dot dot but a dot is escaped") {
      assert(path"${"foo"}/${"bar"}/.\.".posixString == "foo");
    }

## Dependencies

    let { percentEscapeOctetTo } = import("../url");

[CWE-22]: https://cwe.mitre.org/data/definitions/22.html
