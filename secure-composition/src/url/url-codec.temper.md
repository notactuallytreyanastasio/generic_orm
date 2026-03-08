# URL Codec

    export let percentEscapeOctetTo(octet: Int32, sb: StringBuilder): Void {
      sb.append("%");
      appendHex((octet & 0xFF) / 16, sb);
      appendHex(octet & 0xF, sb);
    }

    export let appendHex(n: Int, sb: StringBuilder): Void {
      let i = (n & 0xF);
      sb.appendCodePoint(
        i + (if (i < 10) { char'0' } else { (char'a' - 10) })
      ) orelse panic();
    }
