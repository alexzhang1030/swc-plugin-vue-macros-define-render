# define render macro by swc

[Vue Macro Define-Render](https://github.com/sxzz/unplugin-vue-macros/tree/main/packages/define-render)'s SWC version implementation.

from

```js
import { defineComponent as _defineComponent } from 'vue'
import { openBlock as _openBlock, createElementBlock as _createElementBlock } from "vue"
import { h } from 'vue'

export default _defineComponent({
    __name: 'basic',
    setup(__props) {
      defineRender(() => h('div'))
      return (_ctx, _cache) => {
          return _openBlock(), _createElementBlock("div")
      }
    }
})
```

to

```diff
  import { defineComponent as _defineComponent } from 'vue'
  import { openBlock as _openBlock, createElementBlock as _createElementBlock } from "vue"
  import { h } from 'vue'
  
  export default _defineComponent({
    __name: 'basic',
    setup(__props) {
-     defineRender(() => h('div'))
-       return (_ctx, _cache) => {
-           return _openBlock(), _createElementBlock("div")
-       }
+     ;;
+     return () => h('div')
    }
  })
```
