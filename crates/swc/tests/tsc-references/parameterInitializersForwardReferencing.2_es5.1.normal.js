function _objectWithoutProperties(source, excluded) {
    if (source == null) return {};
    var target = _objectWithoutPropertiesLoose(source, excluded);
    var key, i;
    if (Object.getOwnPropertySymbols) {
        var sourceSymbolKeys = Object.getOwnPropertySymbols(source);
        for(i = 0; i < sourceSymbolKeys.length; i++){
            key = sourceSymbolKeys[i];
            if (excluded.indexOf(key) >= 0) continue;
            if (!Object.prototype.propertyIsEnumerable.call(source, key)) continue;
            target[key] = source[key];
        }
    }
    return target;
}
function _objectWithoutPropertiesLoose(source, excluded) {
    if (source == null) return {};
    var target = {};
    var sourceKeys = Object.keys(source);
    var key, i;
    for(i = 0; i < sourceKeys.length; i++){
        key = sourceKeys[i];
        if (excluded.indexOf(key) >= 0) continue;
        target[key] = source[key];
    }
    return target;
}
// @target: es5, es2015, esnext
// @noEmit: true
// @noTypesAndSymbols: true
// https://github.com/microsoft/TypeScript/issues/36295
function a() {}
function b() {
    var _param = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : a();
    var _b = _param.b, b = _b === void 0 ? a() : _b, x = _objectWithoutProperties(_param, [
        "b"
    ]);
    var a1;
}
var x = "";
function c() {
    var _param = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : a(), d = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : x;
    var b = _param.b, c = _objectWithoutProperties(_param, [
        "b"
    ]);
    var x1;
}
