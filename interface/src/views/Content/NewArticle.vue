<template>
  <div class="create-post-page">
    <h4>Create a new muster protocol</h4>
    <ValidateForm @form-submit="onFormSubmit">
      <div class="mb-3">
	<label class="form-label">Title:</label>
	<ValidateNewTitel
	  :rules="titleRules"
	  v-model="titleVal"
	  placeholder="input your article Title here"
	  type="text"
	  ref="titleRef"
	></ValidateNewTitel>
      </div>
      <div class="mb-3">
	<label class="form-label">Description:</label>
	<ValidateNewInput
	  rows="10"
	  type="text"
	  :rules="contentRules"
	  v-model="contentVal"
	  placeholder="input your article here"
	  ref="contentRef"
	></ValidateNewInput>
	<form>
	  <div class="form-group">
	    <label for="exampleFormControlFile1">File input</label>
	    <br>
	    <input type="file" class="form-control-file" id="exampleFormControlFile1">
	  </div>
	</form>
      </div>
      <template #submit>
	<button class="btn btn-primary btn-large">Upload</button>
      </template>
    </ValidateForm>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import ValidateNewTitel from '../../components/ValidForm/ValidateNewTitle.vue'
import ValidateNewInput from '../../components/ValidForm/ValidateNewInput.vue'
import ValidateForm from '../../components/ValidForm/ValidateForm.vue'
import { useRouter } from 'vue-router'
import { useStore } from 'vuex'
export interface ColumnProps {
  id: number;
  title: string;
  avatar: string;
  description: string;
}

export default defineComponent({
  name: 'NewArticle',
  components: {
    ValidateForm,
    ValidateNewTitel,
    ValidateNewInput
  },
  setup () {
    const router = useRouter()
    const store = useStore()
    const titleRules = store.state.Column.titleRules
    const contentRules = store.state.Column.contentRules
    const titleVal = ref('')
    const contentVal = ref('')
    const titleRef = ref<any>()
    const contentRef = ref<any>()
    const onFormSubmit = () => {
      const newPost: ColumnProps = {
	id: new Date().getTime(),
	title: titleVal.value,
	description: contentVal.value,
	avatar: ''
      }
      if (titleRef.value.validateInput() && contentRef.value.validateInput()) {
	store.commit('Column/creatPost', newPost)
	router.push('/column')
      }
    }
    return {
      titleVal,
      contentVal,
      titleRef,
      contentRef,
      titleRules,
      contentRules,
      onFormSubmit
    }
  }
})
</script>
