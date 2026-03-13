import {
  type as type__780, stdNetSend as stdNetSend_778, panic as panic_786
} from "@temperlang/core";
export class NetRequest extends type__780() {
  /** @type {string} */
  #url_769;
  /** @type {string} */
  #method_770;
  /** @type {string | null} */
  #bodyContent_771;
  /** @type {string | null} */
  #bodyMimeType_772;
  /**
   * @param {string} content_774
   * @param {string} mimeType_775
   */
  post(content_774, mimeType_775) {
    this.#method_770 = "POST";
    this.#bodyContent_771 = content_774;
    let t_776 = this.#bodyMimeType_772;
    this.#bodyMimeType_772 = t_776;
    return;
  }
  /** @returns {globalThis.Promise<NetResponse>} */
  send() {
    return stdNetSend_778(this.#url_769, this.#method_770, this.#bodyContent_771, this.#bodyMimeType_772);
  }
  /** @param {string} url_779 */
  constructor(url_779) {
    super ();
    this.#url_769 = url_779;
    this.#method_770 = "GET";
    this.#bodyContent_771 = null;
    this.#bodyMimeType_772 = null;
    return;
  }
};
export class NetResponse extends type__780() {
};
/**
 * @param {string} url_782
 * @param {string} method_783
 * @param {string | null} bodyContent_784
 * @param {string | null} bodyMimeType_785
 * @returns {globalThis.Promise<NetResponse>}
 */
function sendRequest_781(url_782, method_783, bodyContent_784, bodyMimeType_785) {
  return panic_786();
}
