local temper = require('temper-core');
local NetRequest, NetResponse, sendRequest__16, exports;
NetRequest = temper.type('NetRequest');
NetRequest.methods.post = function(this__0, content__22, mimeType__23)
  local t_276;
  this__0.method__18 = 'POST';
  this__0.bodyContent__19 = content__22;
  t_276 = this__0.bodyMimeType__20;
  this__0.bodyMimeType__20 = t_276;
  return nil;
end;
NetRequest.methods.send = function(this__1)
  return temper.stdnetsend(this__1.url__17, this__1.method__18, this__1.bodyContent__19, this__1.bodyMimeType__20);
end;
NetRequest.constructor = function(this__5, url__28)
  this__5.url__17 = url__28;
  this__5.method__18 = 'GET';
  this__5.bodyContent__19 = temper.null;
  this__5.bodyMimeType__20 = temper.null;
  return nil;
end;
NetResponse = temper.type('NetResponse');
sendRequest__16 = function(url__35, method__36, bodyContent__37, bodyMimeType__38)
  if (bodyContent__37 == nil) then
    bodyContent__37 = temper.null;
  end
  if (bodyMimeType__38 == nil) then
    bodyMimeType__38 = temper.null;
  end
  return temper.bubble();
end;
exports = {};
exports.NetRequest = NetRequest;
exports.NetResponse = NetResponse;
return exports;
