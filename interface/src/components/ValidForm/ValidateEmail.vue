<template>
  <div class="validate-input-container pb-3">
    <input
      class="form-control"
      :class="{'is-invalid': inputRef.error}"
      :value="inputRef.val"
      @blur="validateInput"
      @input="updateValue"
      v-bind="$attrs"
    />
    <span v-if="inputRef.error" class="invalid-feedback">{{inputRef.message}}</span>
  </div>
</template>

<script lang="ts">
import { defineComponent, reactive, PropType } from 'vue'
import { useStore } from 'vuex'
interface RuleProp {
  type: 'required' | 'email' | 'passlength';
  message: string;
}
type RulesProp = RuleProp[]

export default defineComponent({
  name: 'ValidateEmail',
  inheritAttrs: false,
  props: {
    rules: Array as PropType<RulesProp>,
    modelValue: String
  },
  setup (props, context) {
    const store = useStore()
    const inputRef = reactive({
      val: props.modelValue || '',
      error: false,
      message: ''
    })
    const emailReg = store.state.Login.emailReg
    const updateValue = (e: Event) => {
      const targetValue = (e.target as HTMLInputElement).value
      inputRef.val = targetValue
      context.emit('update:modelValue', targetValue)
    }
    const validateInput = () => {
      if (props.rules) {
	const allPassed = props.rules.every(rule => {
	  let passed = true
	  inputRef.message = rule.message
	  switch (rule.type) {
	    case 'required':
	      passed = (inputRef.val.trim() !== '')
	      break
	    case 'email':
	      passed = emailReg.test(inputRef.val)
	      break
	    case 'passlength':
	      passed = (inputRef.val.length > 3)
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
