using T = System.Threading.Tasks;
using N = TemperLang.Std.Net;
namespace TemperLang.Std.Net
{
    public class NetRequest
    {
        readonly string url__17;
        string method__18;
        string ? bodyContent__19;
        string ? bodyMimeType__20;
        public void Post(string content__22, string mimeType__23)
        {
            this.method__18 = "POST";
            this.bodyContent__19 = content__22;
            string ? t___50 = this.bodyMimeType__20;
            this.bodyMimeType__20 = t___50;
        }
        public T::Task<N::INetResponse> Send()
        {
            return N::NetSupport.StdNetSend(this.url__17, this.method__18, this.bodyContent__19, this.bodyMimeType__20);
        }
        public NetRequest(string url__28)
        {
            this.url__17 = url__28;
            this.method__18 = "GET";
            this.bodyContent__19 = null;
            this.bodyMimeType__20 = null;
        }
    }
}
