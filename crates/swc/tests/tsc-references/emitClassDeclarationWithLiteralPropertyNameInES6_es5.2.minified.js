function _defineProperties(target, props) {
    for(var i = 0; i < props.length; i++){
        var descriptor = props[i];
        descriptor.enumerable = descriptor.enumerable || !1, descriptor.configurable = !0, "value" in descriptor && (descriptor.writable = !0), Object.defineProperty(target, descriptor.key, descriptor);
    }
}
var B = function() {
    "use strict";
    var Constructor, protoProps, staticProps;
    function B() {
        !function(instance, Constructor) {
            if (!(instance instanceof Constructor)) throw new TypeError("Cannot call a class as a function");
        }(this, B), this.hello = 10, this[6] = "world", this[10076] = "WORLD", this[20] = "twenty";
    }
    return protoProps = [
        {
            key: "foo",
            value: function() {}
        },
        {
            key: 14,
            value: function() {}
        },
        {
            key: 11,
            value: function() {}
        },
        {
            key: "interface",
            value: function() {}
        }
    ], _defineProperties((Constructor = B).prototype, protoProps), staticProps && _defineProperties(Constructor, staticProps), B;
}();
B.hi = 10000, B[22] = "twenty-two", B[5] = "binary", B[1693] = "octal";
