<template>
  <div class="validate-input-container pb-3">
    <textarea
      class="form-control"
      :class="{'is-invalid': inputRef.error}"
      :value="inputRef.val"
      @blur="validateInput"
      @input="updateValue"
      v-bind="$attrs"
    ></textarea>
    <span v-if="inputRef.error" class="invalid-feedback">{{inputRef.message}}</span>
  </div>
</template>

<script lang="ts">
import { defineComponent, reactive, PropType } from 'vue'
interface RuleProp {
  type: 'required' | 'articlelength';
  message: string;
}
type RulesProp = RuleProp[]
export type TagType = 'input' | 'textarea'

export default defineComponent({
  inheritAttrs: false,
  props: {
    rules: Array as PropType<RulesProp>,
    modelValue: String
  },
  setup (props, context) {
    const inputRef = reactive({
      val: props.modelValue || '',
      error: false,
      message: ''
    })
    const updateValue = (e: Event) => {
      const targetValue = (e.target as HTMLInputElement).value
      inputRef.val = targetValue
      context.emit('update:modelValue', targetValue)
    }
    const validateInput = () => {
      console.log(props.rules)
      if (props.rules) {
	const allPassed = props.rules.every(rule => {
	  let passed = true
	  inputRef.message = rule.message
	  switch (rule.type) {
	    case 'required':
	      passed = (inputRef.val.trim() !== '')
	      break
	    case 'articlelength':
	      passed = (inputRef.val.length < 100)
	      break
	    default:
	      break
	  }
	  return passed
	})
	inputRef.error = !allPassed
	return allPassed
      }
      return false
    }
    return {
      inputRef,
      validateInput,
      updateValue
    }
  }
})
</script>
