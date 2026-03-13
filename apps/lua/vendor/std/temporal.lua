local temper = require('temper-core');
local JsonAdapter, JsonString, DateJsonAdapter__109, Date, daysInMonth__34, isLeapYear__32, padTo__33, dayOfWeekLookupTableLeapy__35, dayOfWeekLookupTableNotLeapy__36, exports;
JsonAdapter = temper.import('std/json', 'JsonAdapter');
JsonString = temper.import('std/json', 'JsonString');
DateJsonAdapter__109 = temper.type('DateJsonAdapter__109', JsonAdapter);
DateJsonAdapter__109.methods.encodeToJson = function(this__120, x__116, p__117)
  x__116:encodeToJson(p__117);
  return nil;
end;
DateJsonAdapter__109.methods.decodeFromJson = function(this__121, t__118, ic__119)
  return Date.decodeFromJson(t__118, ic__119);
end;
DateJsonAdapter__109.constructor = function(this__122)
  return nil;
end;
Date = temper.type('Date');
Date.methods.encodeToJson = function(this__20, p__91)
  local t_242;
  t_242 = temper.date_tostring(this__20);
  p__91:stringValue(t_242);
  return nil;
end;
Date.decodeFromJson = function(t__94, ic__95)
  local t_243;
  t_243 = temper.cast_to(t__94, JsonString);
  return temper.date_fromisostring(t_243.content);
end;
Date.jsonAdapter = function()
  return DateJsonAdapter__109();
end;
daysInMonth__34 = temper.listof(0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31);
isLeapYear__32 = function(year__41)
  local return__21, t_244;
  if (temper.int32_mod(year__41, 4) == 0) then
    if (temper.int32_mod(year__41, 100) ~= 0) then
      return__21 = true;
    else
      t_244 = temper.int32_mod(year__41, 400);
      return__21 = (t_244 == 0);
    end
  else
    return__21 = false;
  end
  return return__21;
end;
padTo__33 = function(minWidth__43, num__44, sb__45)
  local t_245, t_246, t_247, decimal__47, decimalIndex__48, decimalEnd__49, t_248, nNeeded__50;
  decimal__47 = temper.int32_tostring(num__44, 10);
  decimalIndex__48 = 1.0;
  decimalEnd__49 = temper.string_end(decimal__47);
  if temper.stringindexoption_compareto_lt(decimalIndex__48, decimalEnd__49) then
    t_245 = temper.string_get(decimal__47, decimalIndex__48);
    t_247 = (t_245 == 45);
  else
    t_247 = false;
  end
  if t_247 then
    temper.stringbuilder_append(sb__45, '-');
    t_246 = temper.string_next(decimal__47, decimalIndex__48);
    decimalIndex__48 = t_246;
  end
  t_248 = temper.string_countbetween(decimal__47, decimalIndex__48, decimalEnd__49);
  nNeeded__50 = temper.int32_sub(minWidth__43, t_248);
  while (nNeeded__50 > 0) do
    temper.stringbuilder_append(sb__45, '0');
    nNeeded__50 = temper.int32_sub(nNeeded__50, 1);
  end
  temper.stringbuilder_appendbetween(sb__45, decimal__47, decimalIndex__48, decimalEnd__49);
  return nil;
end;
dayOfWeekLookupTableLeapy__35 = temper.listof(0, 0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6);
dayOfWeekLookupTableNotLeapy__36 = temper.listof(0, 0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5);
exports = {};
exports.Date = Date;
return exports;
