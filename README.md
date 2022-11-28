# define render macros by swc

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
