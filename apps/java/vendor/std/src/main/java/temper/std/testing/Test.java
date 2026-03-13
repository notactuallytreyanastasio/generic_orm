package temper.std.testing;
import temper.core.Core;
import java.util.List;
import java.util.function.Supplier;
import temper.core.Nullable;
import java.util.function.Function;
import java.util.ArrayList;
public final class Test {
    public void assert_(boolean success__43, Supplier<String> message__44) {
        String t_406;
        if (!success__43) {
            this._passing = false;
            t_406 = message__44.get();
            Core.listAdd(this._messages, t_406);
        }
    }
    public void assertHard(boolean success__47, Supplier<String> message__48) {
        this.assert_(success__47, message__48);
        if (!success__47) {
            this._failedOnAssert = true;
            Core.throwAssertionError(this.messagesCombined());
        }
    }
    public void softFailToHard() {
        if (this.isHasUnhandledFail()) {
            this._failedOnAssert = true;
            Core.throwAssertionError(this.messagesCombined());
        }
    }
    public boolean isPassing() {
        return this._passing;
    }
    /**
     * Messages access is presented as a function because it likely allocates. Also,
     * messages might be automatically constructed in some cases, so it's possibly
     * unwise to depend on their exact formatting.
     */
    public List<String> messages() {
        return List.copyOf(this._messages);
    }
    public boolean isFailedOnAssert() {
        return this._failedOnAssert;
    }
    public boolean isHasUnhandledFail() {
        boolean t_264;
        if (this._failedOnAssert) {
            t_264 = true;
        } else {
            t_264 = this._passing;
        }
        return !t_264;
    }
    public @Nullable String messagesCombined() {
        @Nullable String return__35;
        if (this._messages.isEmpty()) {
            return__35 = null;
        } else {
            Function<String, String> fn__399 = it__64 -> it__64;
            return__35 = Core.listJoinObj(this._messages, ", ", fn__399);
        }
        return return__35;
    }
    boolean _failedOnAssert;
    boolean _passing;
    final List<String> _messages;
    public Test() {
        this._failedOnAssert = false;
        this._passing = true;
        List<String> t_398 = new ArrayList<>();
        this._messages = t_398;
    }
}
