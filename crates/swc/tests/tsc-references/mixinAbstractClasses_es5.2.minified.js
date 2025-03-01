function _classCallCheck(instance, Constructor) {
    if (!(instance instanceof Constructor)) throw new TypeError("Cannot call a class as a function");
}
function _defineProperties(target, props) {
    for(var i = 0; i < props.length; i++){
        var descriptor = props[i];
        descriptor.enumerable = descriptor.enumerable || !1, descriptor.configurable = !0, "value" in descriptor && (descriptor.writable = !0), Object.defineProperty(target, descriptor.key, descriptor);
    }
}
function _createClass(Constructor, protoProps, staticProps) {
    return protoProps && _defineProperties(Constructor.prototype, protoProps), staticProps && _defineProperties(Constructor, staticProps), Constructor;
}
function _getPrototypeOf(o) {
    return _getPrototypeOf = Object.setPrototypeOf ? Object.getPrototypeOf : function _getPrototypeOf(o) {
        return o.__proto__ || Object.getPrototypeOf(o);
    }, _getPrototypeOf(o);
}
function _inherits(subClass, superClass) {
    if ("function" != typeof superClass && null !== superClass) throw new TypeError("Super expression must either be null or a function");
    subClass.prototype = Object.create(superClass && superClass.prototype, {
        constructor: {
            value: subClass,
            writable: !0,
            configurable: !0
        }
    }), superClass && _setPrototypeOf(subClass, superClass);
}
function _setPrototypeOf(o, p) {
    return _setPrototypeOf = Object.setPrototypeOf || function _setPrototypeOf(o, p) {
        return o.__proto__ = p, o;
    }, _setPrototypeOf(o, p);
}
function _createSuper(Derived) {
    var hasNativeReflectConstruct = function() {
        if ("undefined" == typeof Reflect || !Reflect.construct) return !1;
        if (Reflect.construct.sham) return !1;
        if ("function" == typeof Proxy) return !0;
        try {
            return Boolean.prototype.valueOf.call(Reflect.construct(Boolean, [], function() {})), !0;
        } catch (e) {
            return !1;
        }
    }();
    return function() {
        var obj, self, call, result, Super = _getPrototypeOf(Derived);
        if (hasNativeReflectConstruct) {
            var NewTarget = _getPrototypeOf(this).constructor;
            result = Reflect.construct(Super, arguments, NewTarget);
        } else result = Super.apply(this, arguments);
        return self = this, (call = result) && ("object" == ((obj = call) && "undefined" != typeof Symbol && obj.constructor === Symbol ? "symbol" : typeof obj) || "function" == typeof call) ? call : (function(self) {
            if (void 0 === self) throw new ReferenceError("this hasn't been initialised - super() hasn't been called");
            return self;
        })(self);
    };
}
function Mixin(baseClass1) {
    var MixinClass = function(baseClass) {
        "use strict";
        _inherits(MixinClass, baseClass);
        var _super = _createSuper(MixinClass);
        function MixinClass() {
            return _classCallCheck(this, MixinClass), _super.apply(this, arguments);
        }
        return _createClass(MixinClass, [
            {
                key: "mixinMethod",
                value: function() {}
            }
        ]), MixinClass;
    }(baseClass1);
    return MixinClass;
}
var ConcreteBase = function() {
    "use strict";
    function ConcreteBase() {
        _classCallCheck(this, ConcreteBase);
    }
    return _createClass(ConcreteBase, [
        {
            key: "baseMethod",
            value: function() {}
        }
    ]), ConcreteBase;
}(), AbstractBase = function() {
    "use strict";
    _classCallCheck(this, AbstractBase);
}, DerivedFromConcrete = function(_super) {
    "use strict";
    _inherits(DerivedFromConcrete, _super);
    var _super1 = _createSuper(DerivedFromConcrete);
    function DerivedFromConcrete() {
        return _classCallCheck(this, DerivedFromConcrete), _super1.apply(this, arguments);
    }
    return DerivedFromConcrete;
}(Mixin(ConcreteBase)), wasConcrete = new DerivedFromConcrete();
wasConcrete.baseMethod(), wasConcrete.mixinMethod();
var DerivedFromAbstract = function(_super) {
    "use strict";
    _inherits(DerivedFromAbstract, _super);
    var _super2 = _createSuper(DerivedFromAbstract);
    function DerivedFromAbstract() {
        return _classCallCheck(this, DerivedFromAbstract), _super2.apply(this, arguments);
    }
    return _createClass(DerivedFromAbstract, [
        {
            key: "abstractBaseMethod",
            value: function() {}
        }
    ]), DerivedFromAbstract;
}(Mixin(AbstractBase)), wasAbstract = new DerivedFromAbstract();
wasAbstract.abstractBaseMethod(), wasAbstract.mixinMethod();
