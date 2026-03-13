local temper = require('temper-core');
local RegexNode, Capture, CodePart, CodePoints, Special, SpecialSet, CodeRange, CodeSet, Or, Repeat, Sequence, Match, Group, RegexRefs__56, Regex, RegexFormatter__66, Codes__83, Begin__49, return__192, Begin, Dot__50, return__194, Dot, End__51, return__196, End, WordBoundary__52, return__198, WordBoundary, Digit__53, return__200, Digit, Space__54, return__202, Space, Word__55, return__204, Word, buildEscapeNeeds__163, escapeNeeds__165, regexRefs__164, entire, oneOrMore, optional, exports;
RegexNode = temper.type('RegexNode');
RegexNode.methods.compiled = function(this__44)
  return Regex(this__44);
end;
RegexNode.methods.found = function(this__45, text__172)
  return this__45:compiled():found(text__172);
end;
RegexNode.methods.find = function(this__46, text__175)
  return this__46:compiled():find(text__175);
end;
RegexNode.methods.replace = function(this__47, text__178, format__179)
  return this__47:compiled():replace(text__178, format__179);
end;
RegexNode.methods.split = function(this__48, text__182)
  return this__48:compiled():split(text__182);
end;
Capture = temper.type('Capture', RegexNode);
Capture.constructor = function(this__89, name__187, item__188)
  this__89.name__184 = name__187;
  this__89.item__185 = item__188;
  return nil;
end;
Capture.get.name = function(this__446)
  return this__446.name__184;
end;
Capture.get.item = function(this__449)
  return this__449.item__185;
end;
CodePart = temper.type('CodePart', RegexNode);
CodePoints = temper.type('CodePoints', CodePart);
CodePoints.constructor = function(this__91, value__191)
  this__91.value__189 = value__191;
  return nil;
end;
CodePoints.get.value = function(this__452)
  return this__452.value__189;
end;
Special = temper.type('Special', RegexNode);
SpecialSet = temper.type('SpecialSet', CodePart, Special);
CodeRange = temper.type('CodeRange', CodePart);
CodeRange.constructor = function(this__107, min__209, max__210)
  this__107.min__206 = min__209;
  this__107.max__207 = max__210;
  return nil;
end;
CodeRange.get.min = function(this__455)
  return this__455.min__206;
end;
CodeRange.get.max = function(this__458)
  return this__458.max__207;
end;
CodeSet = temper.type('CodeSet', RegexNode);
CodeSet.constructor = function(this__109, items__214, negated__544)
  local negated__215;
  if temper.is_null(negated__544) then
    negated__215 = false;
  else
    negated__215 = negated__544;
  end
  this__109.items__211 = items__214;
  this__109.negated__212 = negated__215;
  return nil;
end;
CodeSet.get.items = function(this__461)
  return this__461.items__211;
end;
CodeSet.get.negated = function(this__464)
  return this__464.negated__212;
end;
Or = temper.type('Or', RegexNode);
Or.constructor = function(this__112, items__218)
  this__112.items__216 = items__218;
  return nil;
end;
Or.get.items = function(this__467)
  return this__467.items__216;
end;
Repeat = temper.type('Repeat', RegexNode);
Repeat.constructor = function(this__115, item__224, min__225, max__226, reluctant__546)
  local reluctant__227;
  if (max__226 == nil) then
    max__226 = temper.null;
  end
  if temper.is_null(reluctant__546) then
    reluctant__227 = false;
  else
    reluctant__227 = reluctant__546;
  end
  this__115.item__219 = item__224;
  this__115.min__220 = min__225;
  this__115.max__221 = max__226;
  this__115.reluctant__222 = reluctant__227;
  return nil;
end;
Repeat.get.item = function(this__470)
  return this__470.item__219;
end;
Repeat.get.min = function(this__473)
  return this__473.min__220;
end;
Repeat.get.max = function(this__476)
  return this__476.max__221;
end;
Repeat.get.reluctant = function(this__479)
  return this__479.reluctant__222;
end;
Sequence = temper.type('Sequence', RegexNode);
Sequence.constructor = function(this__121, items__238)
  this__121.items__236 = items__238;
  return nil;
end;
Sequence.get.items = function(this__482)
  return this__482.items__236;
end;
Match = temper.type('Match');
Match.constructor = function(this__124, full__242, groups__243)
  this__124.full__239 = full__242;
  this__124.groups__240 = groups__243;
  return nil;
end;
Match.get.full = function(this__497)
  return this__497.full__239;
end;
Match.get.groups = function(this__500)
  return this__500.groups__240;
end;
Group = temper.type('Group');
Group.constructor = function(this__127, name__249, value__250, begin__251, end__252)
  this__127.name__244 = name__249;
  this__127.value__245 = value__250;
  this__127.begin__246 = begin__251;
  this__127.end__247 = end__252;
  return nil;
end;
Group.get.name = function(this__485)
  return this__485.name__244;
end;
Group.get.value = function(this__488)
  return this__488.value__245;
end;
Group.get.begin = function(this__491)
  return this__491.begin__246;
end;
Group.get.end_ = function(this__494)
  return this__494.end__247;
end;
RegexRefs__56 = temper.type('RegexRefs__56');
RegexRefs__56.constructor = function(this__129, codePoints__548, group__550, match__552, orObject__554)
  local t_277, t_278, t_279, t_280, t_281, codePoints__258, group__259, match__260, orObject__261;
  if temper.is_null(codePoints__548) then
    t_277 = CodePoints('');
    codePoints__258 = t_277;
  else
    codePoints__258 = codePoints__548;
  end
  if temper.is_null(group__550) then
    t_278 = Group('', '', 1.0, 1.0);
    group__259 = t_278;
  else
    group__259 = group__550;
  end
  if temper.is_null(match__552) then
    t_279 = temper.map_constructor(temper.listof(temper.pair_constructor('', group__259)));
    t_280 = Match(group__259, t_279);
    match__260 = t_280;
  else
    match__260 = match__552;
  end
  if temper.is_null(orObject__554) then
    t_281 = Or(temper.listof());
    orObject__261 = t_281;
  else
    orObject__261 = orObject__554;
  end
  this__129.codePoints__253 = codePoints__258;
  this__129.group__254 = group__259;
  this__129.match__255 = match__260;
  this__129.orObject__256 = orObject__261;
  return nil;
end;
RegexRefs__56.get.codePoints = function(this__503)
  return this__503.codePoints__253;
end;
RegexRefs__56.get.group = function(this__506)
  return this__506.group__254;
end;
RegexRefs__56.get.match = function(this__509)
  return this__509.match__255;
end;
RegexRefs__56.get.orObject = function(this__512)
  return this__512.orObject__256;
end;
Regex = temper.type('Regex');
Regex.constructor = function(this__57, data__264)
  local t_282, formatted__266, t_283;
  t_282 = data__264;
  this__57.data__262 = t_282;
  formatted__266 = temper.regex_format(data__264);
  t_283 = temper.regex_compileformatted(data__264, formatted__266);
  this__57.compiled__281 = t_283;
  return nil;
end;
Regex.methods.found = function(this__58, text__268)
  return temper.regex_compiledfound(this__58, this__58.compiled__281, text__268);
end;
Regex.methods.find = function(this__59, text__271, begin__556)
  local begin__272;
  if temper.is_null(begin__556) then
    begin__272 = 1.0;
  else
    begin__272 = begin__556;
  end
  return temper.regex_compiledfind(this__59, this__59.compiled__281, text__271, begin__272, regexRefs__164);
end;
Regex.methods.replace = function(this__60, text__275, format__276)
  return temper.regex_compiledreplace(this__60, this__60.compiled__281, text__275, format__276, regexRefs__164);
end;
Regex.methods.split = function(this__61, text__279)
  return temper.regex_compiledsplit(this__61, this__61.compiled__281, text__279, regexRefs__164);
end;
Regex.get.data = function(this__539)
  return this__539.data__262;
end;
RegexFormatter__66 = temper.type('RegexFormatter__66');
RegexFormatter__66.methods.format = function(this__67, regex__312)
  this__67:pushRegex(regex__312);
  return temper.stringbuilder_tostring(this__67.out__303);
end;
RegexFormatter__66.methods.pushRegex = function(this__68, regex__315)
  local t_284, t_285, t_286, t_287, t_288, t_289, t_290;
  if temper.instance_of(regex__315, Capture) then
    t_284 = temper.cast_to(regex__315, Capture);
    this__68:pushCapture(t_284);
  elseif temper.instance_of(regex__315, CodePoints) then
    t_285 = temper.cast_to(regex__315, CodePoints);
    this__68:pushCodePoints(t_285, false);
  elseif temper.instance_of(regex__315, CodeRange) then
    t_286 = temper.cast_to(regex__315, CodeRange);
    this__68:pushCodeRange(t_286);
  elseif temper.instance_of(regex__315, CodeSet) then
    t_287 = temper.cast_to(regex__315, CodeSet);
    this__68:pushCodeSet(t_287);
  elseif temper.instance_of(regex__315, Or) then
    t_288 = temper.cast_to(regex__315, Or);
    this__68:pushOr(t_288);
  elseif temper.instance_of(regex__315, Repeat) then
    t_289 = temper.cast_to(regex__315, Repeat);
    this__68:pushRepeat(t_289);
  elseif temper.instance_of(regex__315, Sequence) then
    t_290 = temper.cast_to(regex__315, Sequence);
    this__68:pushSequence(t_290);
  elseif temper.generic_eq(regex__315, Begin) then
    temper.stringbuilder_append(this__68.out__303, '^');
  elseif temper.generic_eq(regex__315, Dot) then
    temper.stringbuilder_append(this__68.out__303, '.');
  elseif temper.generic_eq(regex__315, End) then
    temper.stringbuilder_append(this__68.out__303, '$');
  elseif temper.generic_eq(regex__315, WordBoundary) then
    temper.stringbuilder_append(this__68.out__303, '\\b');
  elseif temper.generic_eq(regex__315, Digit) then
    temper.stringbuilder_append(this__68.out__303, '\\d');
  elseif temper.generic_eq(regex__315, Space) then
    temper.stringbuilder_append(this__68.out__303, '\\s');
  elseif temper.generic_eq(regex__315, Word) then
    temper.stringbuilder_append(this__68.out__303, '\\w');
  end
  return nil;
end;
RegexFormatter__66.methods.pushCapture = function(this__69, capture__318)
  local t_291, t_292, t_293;
  temper.stringbuilder_append(this__69.out__303, '(');
  t_291 = this__69.out__303;
  t_292 = capture__318.name;
  temper.regexformatter_pushcapturename(this__69, t_291, t_292);
  t_293 = capture__318.item;
  this__69:pushRegex(t_293);
  temper.stringbuilder_append(this__69.out__303, ')');
  return nil;
end;
RegexFormatter__66.methods.pushCode = function(this__71, code__325, insideCodeSet__326)
  local return__148, t_294, t_295, t_296, t_297, t_298, t_299, t_300, t_301, t_302, local_303, local_304, local_305;
  ::continue_25::local_303, local_304, local_305 = temper.pcall(function()
    local specialEscape__328;
    if (code__325 == Codes__83.carriageReturn) then
      specialEscape__328 = 'r';
    elseif (code__325 == Codes__83.newline) then
      specialEscape__328 = 'n';
    elseif (code__325 == Codes__83.tab) then
      specialEscape__328 = 't';
    else
      specialEscape__328 = '';
    end
    if temper.str_ne(specialEscape__328, '') then
      temper.stringbuilder_append(this__71.out__303, '\\');
      temper.stringbuilder_append(this__71.out__303, specialEscape__328);
      return__148 = nil;
      return 'break_24', 'flow';
    end
    if (code__325 <= 127) then
      local escapeNeed__329;
      escapeNeed__329 = temper.list_get(escapeNeeds__165, code__325);
      if temper.generic_eq(escapeNeed__329, 2) then
        t_295 = true;
      else
        if insideCodeSet__326 then
          t_294 = (code__325 == Codes__83.dash);
        else
          t_294 = false;
        end
        t_295 = t_294;
      end
      if t_295 then
        temper.stringbuilder_append(this__71.out__303, '\\');
        t_296 = temper.string_fromcodepoint(code__325);
        temper.stringbuilder_append(this__71.out__303, t_296);
        return__148 = nil;
        return 'break_24', 'flow';
      elseif temper.generic_eq(escapeNeed__329, 0) then
        t_297 = temper.string_fromcodepoint(code__325);
        temper.stringbuilder_append(this__71.out__303, t_297);
        return__148 = nil;
        return 'break_24', 'flow';
      end
    end
    if (code__325 >= Codes__83.supplementalMin) then
      t_301 = true;
    else
      if (code__325 > Codes__83.highControlMax) then
        if (Codes__83.surrogateMin <= code__325) then
          t_298 = (code__325 <= Codes__83.surrogateMax);
        else
          t_298 = false;
        end
        if t_298 then
          t_299 = true;
        else
          t_299 = (code__325 == Codes__83.uint16Max);
        end
        t_300 = not t_299;
      else
        t_300 = false;
      end
      t_301 = t_300;
    end
    if t_301 then
      t_302 = temper.string_fromcodepoint(code__325);
      temper.stringbuilder_append(this__71.out__303, t_302);
    else
      temper.regexformatter_pushcodeto(this__71, this__71.out__303, code__325, insideCodeSet__326);
    end
  end);
  if local_303 then
    if (local_305 == 'flow') then
      if (local_304 == nil) then
      elseif (local_304 == 'break_24') then
        goto break_24;
      end
    end
  else
    temper.bubble();
  end
  return__148 = nil;
  ::break_24::return return__148;
end;
RegexFormatter__66.methods.pushCodePoints = function(this__73, codePoints__336, insideCodeSet__337)
  local t_307, t_308, value__339, index__340;
  value__339 = codePoints__336.value;
  index__340 = 1.0;
  while true do
    if not temper.string_hasindex(value__339, index__340) then
      break;
    end
    t_307 = temper.string_get(value__339, index__340);
    this__73:pushCode(t_307, insideCodeSet__337);
    t_308 = temper.string_next(value__339, index__340);
    index__340 = t_308;
  end
  return nil;
end;
RegexFormatter__66.methods.pushCodeRange = function(this__74, codeRange__342)
  temper.stringbuilder_append(this__74.out__303, '[');
  this__74:pushCodeRangeUnwrapped(codeRange__342);
  temper.stringbuilder_append(this__74.out__303, ']');
  return nil;
end;
RegexFormatter__66.methods.pushCodeRangeUnwrapped = function(this__75, codeRange__345)
  local t_309, t_310;
  t_309 = codeRange__345.min;
  this__75:pushCode(t_309, true);
  temper.stringbuilder_append(this__75.out__303, '-');
  t_310 = codeRange__345.max;
  this__75:pushCode(t_310, true);
  return nil;
end;
RegexFormatter__66.methods.pushCodeSet = function(this__76, codeSet__348)
  local t_311, t_312, t_313, adjusted__350;
  adjusted__350 = temper.regexformatter_adjustcodeset(this__76, codeSet__348, regexRefs__164);
  if temper.instance_of(adjusted__350, CodeSet) then
    t_313 = temper.cast_to(adjusted__350, CodeSet);
    if temper.listed_isempty(t_313.items) then
      if t_313.negated then
        temper.stringbuilder_append(this__76.out__303, '[\\s\\S]');
      else
        temper.stringbuilder_append(this__76.out__303, '(?:$.)');
      end
    else
      local i__351;
      temper.stringbuilder_append(this__76.out__303, '[');
      if t_313.negated then
        temper.stringbuilder_append(this__76.out__303, '^');
      end
      i__351 = 0;
      while true do
        t_311 = temper.list_length(t_313.items);
        if not (i__351 < t_311) then
          break;
        end
        t_312 = temper.list_get(t_313.items, i__351);
        this__76:pushCodeSetItem(t_312);
        i__351 = temper.int32_add(i__351, 1);
      end
      temper.stringbuilder_append(this__76.out__303, ']');
    end
  else
    this__76:pushRegex(adjusted__350);
  end
  return nil;
end;
RegexFormatter__66.methods.pushCodeSetItem = function(this__78, codePart__357)
  local t_314, t_315, t_316;
  if temper.instance_of(codePart__357, CodePoints) then
    t_314 = temper.cast_to(codePart__357, CodePoints);
    this__78:pushCodePoints(t_314, true);
  elseif temper.instance_of(codePart__357, CodeRange) then
    t_315 = temper.cast_to(codePart__357, CodeRange);
    this__78:pushCodeRangeUnwrapped(t_315);
  elseif temper.instance_of(codePart__357, SpecialSet) then
    t_316 = temper.cast_to(codePart__357, SpecialSet);
    this__78:pushRegex(t_316);
  end
  return nil;
end;
RegexFormatter__66.methods.pushOr = function(this__79, or__360)
  local t_317, t_318, t_319;
  if not temper.listed_isempty(or__360.items) then
    local i__362;
    temper.stringbuilder_append(this__79.out__303, '(?:');
    t_317 = temper.list_get(or__360.items, 0);
    this__79:pushRegex(t_317);
    i__362 = 1;
    while true do
      t_318 = temper.list_length(or__360.items);
      if not (i__362 < t_318) then
        break;
      end
      temper.stringbuilder_append(this__79.out__303, '|');
      t_319 = temper.list_get(or__360.items, i__362);
      this__79:pushRegex(t_319);
      i__362 = temper.int32_add(i__362, 1);
    end
    temper.stringbuilder_append(this__79.out__303, ')');
  end
  return nil;
end;
RegexFormatter__66.methods.pushRepeat = function(this__80, repeat__364)
  local t_320, t_321, t_322, t_323, t_324, t_325, min__366, max__367;
  temper.stringbuilder_append(this__80.out__303, '(?:');
  t_325 = repeat__364.item;
  this__80:pushRegex(t_325);
  temper.stringbuilder_append(this__80.out__303, ')');
  min__366 = repeat__364.min;
  max__367 = repeat__364.max;
  if (min__366 == 0) then
    t_322 = (max__367 == 1);
  else
    t_322 = false;
  end
  if t_322 then
    temper.stringbuilder_append(this__80.out__303, '?');
  else
    if (min__366 == 0) then
      t_323 = temper.is_null(max__367);
    else
      t_323 = false;
    end
    if t_323 then
      temper.stringbuilder_append(this__80.out__303, '*');
    else
      if (min__366 == 1) then
        t_324 = temper.is_null(max__367);
      else
        t_324 = false;
      end
      if t_324 then
        temper.stringbuilder_append(this__80.out__303, '+');
      else
        t_320 = temper.int32_tostring(min__366);
        temper.stringbuilder_append(this__80.out__303, temper.concat('{', t_320));
        if (min__366 ~= max__367) then
          temper.stringbuilder_append(this__80.out__303, ',');
          if not temper.is_null(max__367) then
            t_321 = temper.int32_tostring(max__367);
            temper.stringbuilder_append(this__80.out__303, t_321);
          end
        end
        temper.stringbuilder_append(this__80.out__303, '}');
      end
    end
  end
  if repeat__364.reluctant then
    temper.stringbuilder_append(this__80.out__303, '?');
  end
  return nil;
end;
RegexFormatter__66.methods.pushSequence = function(this__81, sequence__369)
  local t_326, t_327, i__371;
  i__371 = 0;
  while true do
    t_326 = temper.list_length(sequence__369.items);
    if not (i__371 < t_326) then
      break;
    end
    t_327 = temper.list_get(sequence__369.items, i__371);
    this__81:pushRegex(t_327);
    i__371 = temper.int32_add(i__371, 1);
  end
  return nil;
end;
RegexFormatter__66.methods.maxCode = function(this__82, codePart__373)
  local return__159, t_328, t_329;
  if temper.instance_of(codePart__373, CodePoints) then
    local value__375;
    t_329 = temper.cast_to(codePart__373, CodePoints);
    value__375 = t_329.value;
    if temper.string_isempty(value__375) then
      return__159 = temper.null;
    else
      local max__376, index__377;
      max__376 = 0;
      index__377 = 1.0;
      while true do
        local next__378;
        if not temper.string_hasindex(value__375, index__377) then
          break;
        end
        next__378 = temper.string_get(value__375, index__377);
        if (next__378 > max__376) then
          max__376 = next__378;
        end
        t_328 = temper.string_next(value__375, index__377);
        index__377 = t_328;
      end
      return__159 = max__376;
    end
  elseif temper.instance_of(codePart__373, CodeRange) then
    return__159 = (temper.cast_to(codePart__373, CodeRange)).max;
  elseif temper.generic_eq(codePart__373, Digit) then
    return__159 = Codes__83.digit9;
  elseif temper.generic_eq(codePart__373, Space) then
    return__159 = Codes__83.space;
  elseif temper.generic_eq(codePart__373, Word) then
    return__159 = Codes__83.lowerZ;
  else
    return__159 = temper.null;
  end
  return return__159;
end;
RegexFormatter__66.constructor = function(this__140)
  local t_330;
  t_330 = temper.stringbuilder_constructor();
  this__140.out__303 = t_330;
  return nil;
end;
Codes__83 = temper.type('Codes__83');
Codes__83.constructor = function(this__161)
  return nil;
end;
Codes__83.ampersand = 38;
Codes__83.backslash = 92;
Codes__83.caret = 94;
Codes__83.carriageReturn = 13;
Codes__83.curlyLeft = 123;
Codes__83.curlyRight = 125;
Codes__83.dash = 45;
Codes__83.dot = 46;
Codes__83.highControlMin = 127;
Codes__83.highControlMax = 159;
Codes__83.digit0 = 48;
Codes__83.digit9 = 57;
Codes__83.lowerA = 97;
Codes__83.lowerZ = 122;
Codes__83.newline = 10;
Codes__83.peso = 36;
Codes__83.pipe = 124;
Codes__83.plus = 43;
Codes__83.question = 63;
Codes__83.roundLeft = 40;
Codes__83.roundRight = 41;
Codes__83.slash = 47;
Codes__83.squareLeft = 91;
Codes__83.squareRight = 93;
Codes__83.star = 42;
Codes__83.tab = 9;
Codes__83.tilde = 42;
Codes__83.upperA = 65;
Codes__83.upperZ = 90;
Codes__83.space = 32;
Codes__83.surrogateMin = 55296;
Codes__83.surrogateMax = 57343;
Codes__83.supplementalMin = 65536;
Codes__83.uint16Max = 65535;
Codes__83.underscore = 95;
Begin__49 = temper.type('Begin__49', Special);
Begin__49.constructor = function(this__93)
  return nil;
end;
return__192 = Begin__49();
Begin = return__192;
Dot__50 = temper.type('Dot__50', Special);
Dot__50.constructor = function(this__95)
  return nil;
end;
return__194 = Dot__50();
Dot = return__194;
End__51 = temper.type('End__51', Special);
End__51.constructor = function(this__97)
  return nil;
end;
return__196 = End__51();
End = return__196;
WordBoundary__52 = temper.type('WordBoundary__52', Special);
WordBoundary__52.constructor = function(this__99)
  return nil;
end;
return__198 = WordBoundary__52();
WordBoundary = return__198;
Digit__53 = temper.type('Digit__53', SpecialSet);
Digit__53.constructor = function(this__101)
  return nil;
end;
return__200 = Digit__53();
Digit = return__200;
Space__54 = temper.type('Space__54', SpecialSet);
Space__54.constructor = function(this__103)
  return nil;
end;
return__202 = Space__54();
Space = return__202;
Word__55 = temper.type('Word__55', SpecialSet);
Word__55.constructor = function(this__105)
  return nil;
end;
return__204 = Word__55();
Word = return__204;
buildEscapeNeeds__163 = function()
  local t_331, t_332, t_333, t_334, t_335, t_336, t_337, t_338, t_339, t_340, t_341, t_342, t_343, t_344, t_345, t_346, t_347, t_348, t_349, t_350, t_351, t_352, t_353, t_354, t_355, escapeNeeds__381, code__382;
  escapeNeeds__381 = temper.listbuilder_constructor();
  code__382 = 0;
  while (code__382 <= 127) do
    if (code__382 == Codes__83.dash) then
      t_338 = true;
    else
      if (code__382 == Codes__83.space) then
        t_337 = true;
      else
        if (code__382 == Codes__83.underscore) then
          t_336 = true;
        else
          if (Codes__83.digit0 <= code__382) then
            t_331 = (code__382 <= Codes__83.digit9);
          else
            t_331 = false;
          end
          if t_331 then
            t_335 = true;
          else
            if (Codes__83.upperA <= code__382) then
              t_332 = (code__382 <= Codes__83.upperZ);
            else
              t_332 = false;
            end
            if t_332 then
              t_334 = true;
            else
              if (Codes__83.lowerA <= code__382) then
                t_333 = (code__382 <= Codes__83.lowerZ);
              else
                t_333 = false;
              end
              t_334 = t_333;
            end
            t_335 = t_334;
          end
          t_336 = t_335;
        end
        t_337 = t_336;
      end
      t_338 = t_337;
    end
    if t_338 then
      t_355 = 0;
    else
      if (code__382 == Codes__83.ampersand) then
        t_354 = true;
      else
        if (code__382 == Codes__83.backslash) then
          t_353 = true;
        else
          if (code__382 == Codes__83.caret) then
            t_352 = true;
          else
            if (code__382 == Codes__83.curlyLeft) then
              t_351 = true;
            else
              if (code__382 == Codes__83.curlyRight) then
                t_350 = true;
              else
                if (code__382 == Codes__83.dot) then
                  t_349 = true;
                else
                  if (code__382 == Codes__83.peso) then
                    t_348 = true;
                  else
                    if (code__382 == Codes__83.pipe) then
                      t_347 = true;
                    else
                      if (code__382 == Codes__83.plus) then
                        t_346 = true;
                      else
                        if (code__382 == Codes__83.question) then
                          t_345 = true;
                        else
                          if (code__382 == Codes__83.roundLeft) then
                            t_344 = true;
                          else
                            if (code__382 == Codes__83.roundRight) then
                              t_343 = true;
                            else
                              if (code__382 == Codes__83.slash) then
                                t_342 = true;
                              else
                                if (code__382 == Codes__83.squareLeft) then
                                  t_341 = true;
                                else
                                  if (code__382 == Codes__83.squareRight) then
                                    t_340 = true;
                                  else
                                    if (code__382 == Codes__83.star) then
                                      t_339 = true;
                                    else
                                      t_339 = (code__382 == Codes__83.tilde);
                                    end
                                    t_340 = t_339;
                                  end
                                  t_341 = t_340;
                                end
                                t_342 = t_341;
                              end
                              t_343 = t_342;
                            end
                            t_344 = t_343;
                          end
                          t_345 = t_344;
                        end
                        t_346 = t_345;
                      end
                      t_347 = t_346;
                    end
                    t_348 = t_347;
                  end
                  t_349 = t_348;
                end
                t_350 = t_349;
              end
              t_351 = t_350;
            end
            t_352 = t_351;
          end
          t_353 = t_352;
        end
        t_354 = t_353;
      end
      if t_354 then
        t_355 = 2;
      else
        t_355 = 1;
      end
    end
    temper.listbuilder_add(escapeNeeds__381, t_355);
    code__382 = temper.int32_add(code__382, 1);
  end
  return temper.listbuilder_tolist(escapeNeeds__381);
end;
escapeNeeds__165 = buildEscapeNeeds__163();
regexRefs__164 = RegexRefs__56();
entire = function(item__228)
  return Sequence(temper.listof(Begin, item__228, End));
end;
oneOrMore = function(item__230, reluctant__558)
  local reluctant__231;
  if temper.is_null(reluctant__558) then
    reluctant__231 = false;
  else
    reluctant__231 = reluctant__558;
  end
  return Repeat(item__230, 1, temper.null, reluctant__231);
end;
optional = function(item__233, reluctant__560)
  local reluctant__234;
  if temper.is_null(reluctant__560) then
    reluctant__234 = false;
  else
    reluctant__234 = reluctant__560;
  end
  return Repeat(item__233, 0, 1, reluctant__234);
end;
exports = {};
exports.RegexNode = RegexNode;
exports.Capture = Capture;
exports.CodePart = CodePart;
exports.CodePoints = CodePoints;
exports.Special = Special;
exports.SpecialSet = SpecialSet;
exports.CodeRange = CodeRange;
exports.CodeSet = CodeSet;
exports.Or = Or;
exports.Repeat = Repeat;
exports.Sequence = Sequence;
exports.Match = Match;
exports.Group = Group;
exports.Regex = Regex;
exports.Begin = Begin;
exports.Dot = Dot;
exports.End = End;
exports.WordBoundary = WordBoundary;
exports.Digit = Digit;
exports.Space = Space;
exports.Word = Word;
exports.entire = entire;
exports.oneOrMore = oneOrMore;
exports.optional = optional;
return exports;
