# Contextual Autoescaping Framework

This defines a framework for building contextual autoescaping
accumulators that can be highly statically optimized because the
separate steps, listed below, are all pure functional operators that
can be inlined and simplified at compile time.

Escaper is for transforms from untrusted values to trustworthy values.

They should be stateless, only useful for their behaviour, unless they
represent the composition of two more escapers in which case they should
have two properties: `first` and `second` indicating the application order.

    export interface Escaper {
      // One or more apply methods like
      // apply(interpolation: String): OUT;
    }

Context encapsulates a state in a streaming parser.  It's used both to
parse the fixed part of the content, and to pick escapers to preserve
the meaning of subsequent fixed parts.

    export interface Context {
      public toString(): String;
    }

Autoesc bundles together a context and a subsidiary automaton.

    export class AutoescState<CONTEXT extends Context, ESC extends Escaper>(
      public context: CONTEXT,
      public subsidiary: Subsidiary<ESC>?,
    ) {
      public toString(): String {
        "AutoescState(${context}, ${subsidiary})"
      }
    }

AfterPropagate bundles up information about propagating context across a fixed part,
and context transitions needed to move into a state where an interpolation is valid.

    export class AfterPropagate<CONTEXT extends Context, ESC extends Escaper>(
      public adjustedString: String,
      public consumed: StringIndex,
      public contextAfter: CONTEXT,
      public subsidiaryAfter: Subsidiary<ESC>?,
    ) {
      public get stateAfter: AutoescState<CONTEXT, ESC> {
        new AutoescState(contextAfter, subsidiaryAfter)
      }

push specifies a delegate for a nested language.
The *Codec* is used to decode content before feeding it to the delegate, and to
reencode the delegate's adjusted output.

      public start(delegate: Delegate<ESC>, codec: Codec): AfterPropagate<CONTEXT, ESC> {
        new AfterPropagate<CONTEXT, ESC>(
          adjustedString,
          consumed,
          contextAfter,
          new Subsidiary(delegate, codec),
        )
      }

pop undoes a push by removing a previously pushed delegate when a
nested language region ends.

      public end(): AfterPropagate<CONTEXT, ESC> {
        new AfterPropagate<CONTEXT, ESC>(adjustedString, consumed, contextAfter, null)
      }

      public toString(): String {
        "AfterPropagate(${adjustedString}, consumed, ${stateAfter})"
      }
    }

ContextPropagator knows how to consumes a chunk of context from a literalPart and transition
to the Context after it.

    export interface ContextPropagator<CONTEXT extends Context, ESC extends Escaper> {

after computes the context after processing the literalPart and any
adjustments made to the literal part.

If literalPart is null, then we're doing a context transition to get
into a state where we're ready to interpolate an untrusted value.

      after(
        before: AutoescState<CONTEXT, ESC>,
        literalPart: String?,
        callout: DiagnosticCallout?,
      ): AfterPropagate<CONTEXT, ESC>;
    }

Delegate can represent an automaton for a nested language that can
propagates its own state over strings that relate to chunks of content
from the outer language.  The content can be decoded via a *Codec*.

    export interface Delegate<ESC extends Escaper> {
      escaper(outer: ESC, callout: DiagnosticCallout?): ESC;
      process(known: String?, callout: DiagnosticCallout?): String;
      toString(): String;

close may report problems with the delegate finishing in a bad end state.
For example, if a CSS delegate ends inside a quoted string, that's
indicative that the template is missing a close quote.

      close(callout: DiagnosticCallout?): Void;
    }

ContextDelegate is for delegates that have their own contexts which
are propagated like a regular contextual auto-escaper.

    export interface ContextDelegate<CONTEXT extends Context, ESC extends Escaper> extends Delegate<ESC> {
      get state(): AutoescState<CONTEXT, ESC>;
      set state(x: AutoescState<CONTEXT, ESC>): Void;
      get contextPropagator(): ContextPropagator<CONTEXT, ESC>;
      get sameState(): fn (AutoescState<CONTEXT, ESC>, AutoescState<CONTEXT, ESC>): Boolean;

      process(known: String?, callout: DiagnosticCallout?): String {
        let after: AfterPropagate<CONTEXT, ESC> =
            propagateOver(contextPropagator, this.state, known, sameState, callout);
        this.state = after.stateAfter;
        after.adjustedString
      }
    }

(*OUT* is the representation of the trustworthy output and must be
accepted by with the collector type's *append* method.)

propagateOver is used by both the accumulators and context delegates to propagate context over
a known safe chunk.  Propagation is usual defined in terms of transition tables, and each
transition consumes some prefix of the remaining content, so this does enough transitions to
process the entire chunk.

    export let propagateOver<CONTEXT extends Context, ESC extends Escaper>(
      contextPropagator: ContextPropagator<CONTEXT, ESC>,
      before: AutoescState<CONTEXT, ESC>,
      known: String?,
      sameState: fn (AutoescState<CONTEXT, ESC>, AutoescState<CONTEXT, ESC>): Boolean,
      callout: DiagnosticCallout?,
    ): AfterPropagate<CONTEXT, ESC> {
      var remainder = if (known != null) {
        known
      } else {
        ""
      }
      let afterKnown = remainder.end;

      // (known == null) means that we're preparing for an interpolation.
      // We need to iterate until the state is steady.
      // Otherwise, we're iterating until remainder is empty.
      let consumeRemainder = known != null;

      let adjusted = new StringBuilder();
      var state = before;
      if (!consumeRemainder || !remainder.isEmpty) {
        // pending is where we gather adjusted content to be fed to the delegate.
        var pending = new StringBuilder();
        var sub = state.subsidiary;
        while (true) {
          let known = if (consumeRemainder) { remainder } else { null };
          let after = contextPropagator.after(state, known, callout);
          let contextAfter = after.contextAfter;
          let subAfter = after.subsidiaryAfter;

          let subBefore = sub;
          sub = subAfter;

          let adjustedString = after.adjustedString;
          if (subBefore == null) {
            adjusted.append(adjustedString);
          } else if (subBefore == subAfter) {
            pending.append(adjustedString);
          } else {
            adjusted.append(feedSubsidiary(subBefore, pending.toString(), false, callout));
            subBefore.delegate.close(callout);
            pending = new StringBuilder();
            (if (subAfter != null) { pending } else { adjusted })
              .append(adjustedString);
          }

          let stateAfter = new AutoescState(contextAfter, subAfter);
          let done = if (consumeRemainder) {
            remainder = remainder.slice(after.consumed, remainder.end);
            remainder.isEmpty
          } else {
            sameState(state, stateAfter)
          }

          state = stateAfter;
          if (done) { break; } // Consume all input / reached a fixed point
        }
        let subAtEnd = sub;
        if (subAtEnd != null) {
          adjusted.append(feedSubsidiary(subAtEnd, pending.toString(), !consumeRemainder, callout));
        }
      }

      new AfterPropagate(adjusted.toString(), afterKnown, state.context, state.subsidiary)
    }

EscaperPicker returns the escaper that needs to be applied to untrusted parts to render them
safe in context.

    export interface EscaperPicker<CONTEXT extends Context, ESC extends Escaper> {
      escaperFor(before: AutoescState<CONTEXT, ESC>, callout: DiagnosticCallout?): ESC;
    }

CONTEXT is a type for context values that track the parser context of
the accumulated prefix.  ESC is the type used to represent escapers
that convert interpolated expression values to PARTS.

Additionally, there are two names below that are mentioned in static
conventions but which need not be represented as type parameters.
COLLECTOR is a mutable type that collects safe and unsafe parts and
which is used to produce the output.  OUT is the type of the final
accumulated value.

    export interface ContextualAutoescapingAccumulator<CONTEXT extends Context, ESC extends Escaper> {
      // By convention, these are defined in concrete accumulator types.
      // public static initialState(): AutoescState<CONTEXT> { ... }
      // public static newCollector(): COLLECTOR { ... }
      // public static propagator(): ContextPropagator<CONTEXT> { ... }
      // public static picker(): EscaperPicker<CONTEXT, PART, ESC> { ... }
      // public static fromCollector(collector: COLLECTOR): OUT;
      // public static mergeStates(a: AutoescState<CONTEXT>, b: AutoescState<CONTEXT>, callout: DiagnosticCallout?): AutoescState<CONTEXT>;
      // public static sameStatePredicate(): fn (AutoescState<CONTEXT>, AutoescState<CONTEXT>): Boolean;
      // public static checkEndState(): fn (AutoescState<CONTEXT>, callout: DiagnosticCallout?): Void;
      // - state is initialized to initialState()
      // - The internal collector property is initialized to newCollector()
      // - contextPropagator() just returns propagator().
      // - escaperPicker() just returns picker().
      // - get accumulated() just returns fromCollector(collector)
      // - get sameState() just returns sameStatePredicate().

      public get state(): AutoescState<CONTEXT, ESC>;
      public set state(newState: AutoescState<CONTEXT, ESC>): Void;
      public get sameState(): (fn (AutoescState<CONTEXT, ESC>, AutoescState<CONTEXT, ESC>): Boolean);

      public get escaperPicker(): EscaperPicker<CONTEXT, ESC>;
      public get contextPropagator(): ContextPropagator<CONTEXT, ESC>;

      public prepareForAppend(): ESC {
        if (DEBUG) {
          console.log("prepare, before: ${context.toString()}");
        }
        let after: AfterPropagate<CONTEXT, ESC> = propagateOver(contextPropagator, this.state, null, sameState, null);
        this.state = after.stateAfter;
        let adjusted = after.adjustedString;
        if (!adjusted.isEmpty) {
          collectFixed(adjusted);
        }
        if (DEBUG) {
          console.log("-> ${context.toString()}");
        }
        escaperPicker.escaperFor(this.state, null)
      }

      // Need one or more append methods

      public appendSafe(known: String): Void {
        if (DEBUG) {
          console.log("appendSafe: `${known}`, before: ${context}");
        }
        let after: AfterPropagate<CONTEXT, ESC> = propagateOver(contextPropagator, this.state, known, sameState, null);
        this.state = after.stateAfter;
        let adjusted = after.adjustedString;
        if (!adjusted.isEmpty) {
          collectFixed(adjusted);
        }
        if (DEBUG) {
          console.log("-> ${context}");
        }
      }

      /** Appends the fixed, trusted fragment to the collector. */
      public collectFixed(fixedFragment: String): Void;
    }

feed allows a subsidiary language handler a chance to update its own
context and perhaps offer its own adjustments.

For example, an HTML state machine might use a subsidiary CSS state
machine to process the body of a `<style>` element, but still retain
control over identifying the `</style>` end tag which ends the
element.

    let feedSubsidiary<ESC extends Escaper>(
      subsidiary: Subsidiary<ESC>,
      adjustedStr: String,
      prepareForInterp: Boolean,
      callout: DiagnosticCallout?
    ): String {
      var str = adjustedStr;

      let delegate = subsidiary.delegate;
      let codec = subsidiary.codec;

      str = codec.decode(str);

      str = delegate.process(str, callout);
      if (prepareForInterp) {
        str = "${str}${delegate.process(null, callout)}";
      }

      if (codec != null) {
        str = codec.encode(str);
      }

      str
    }

Codecs allow encoding and decoding a string according to some escaping
convention.
Sometimes we need to decode strings before passing to a subsidiary state
machine and re-encode any strings that they adjust.

    export interface Codec {
      encode(s: String): String;
      decode(s: String): String;
    }

    export class Subsidiary<ESC extends Escaper>(
      public delegate: Delegate<ESC>,
      public codec: Codec,
    ) {
      public toString(): String {
        "Subsidiary(${delegate}, codec)"
      }
    }

    let DEBUG = false;
