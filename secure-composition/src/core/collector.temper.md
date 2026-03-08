# Simple Collector Implementation

Collector keeps an internal mutable list of parts and distinguishes
between fixed and interpolated parts.

It can be used as a basis for text-based auto-escaping accumulators to
interleave fixed parts and properly escaped interpolations.

    export class Collector<PART> {
      private let partsBuilder: ListBuilder<Collected<PART>> = new ListBuilder();
      public appendSafe(fixed: String): Void {
        partsBuilder.add(new CollectedFixed<PART>(fixed));
      }
      public append(part: PART): Void {
        partsBuilder.add(new CollectedPart<PART>(part));
      }
      public get parts(): List<Collected<PART>> { partsBuilder.toList() }
    }

    export sealed interface Collected<PART> {}
    export class CollectedFixed<PART>(public safeText: String) extends Collected<PART> {}
    export class CollectedPart<PART>(public part: PART) extends Collected<PART> {}
