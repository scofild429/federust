<template>

  <div style="background: #61d1d1; box-shadow: 12px 12px 2px 1px rgba(0, 0, 255, .2);">
    <div class="card">
      <div class="card-header">
	<h5>Please reach out to us here if you have any questions. We are also happy to give you a personal demo on request. Thank you. </h5>
      </div>
      <ul class="list-group list-group-flush">
	<li class="list-group-item">
	  <div class="row justify-content-around">
	    <div class="col-md-5">
	      <input
		v-model="data.Name"
		style="width: 100%"
		type="text"
		class="form-control boxxx"
		placeholder="Name"
		required
	      />
	      </div>
	      <div class="col-md-5">
		<input
		  v-model="data.Email"
		  style="width: 100%"
		  type="email"
		  class="form-control boxxx"
		  placeholder="Email"
		  required
		/>
	      </div>
	  </div>
	</li>
	<li class="list-group-item">
	  <ValidateNewInput
	    rows="5"
	    type="text"
	    v-model="data.Inputmessage"
	    placeholder="please input your message here"
	    ref="contentRef"
	  ></ValidateNewInput>
	</li>
      </ul>
      <button class="btn btn-primary" @click.prevent="sendusemail" >Send us email</button>
    </div>
  </div>
</template>
<script lang="ts">
 import {defineComponent, reactive} from "vue";
 import ValidateNewInput from '../../components/ValidForm/ValidateNewInput.vue'
 import axios from "axios";
 import {useRouter} from "vue-router";
 
 export default defineComponent({
   name: "Contactus",
   components: {
     ValidateNewInput,
   },
   setup(){
     const data = reactive({
       Name:'',
       Email: '',
       Inputmessage: ''
     })
     const route = useRouter();
     const sendusemail = async () => {
       try {
	 const backdata = await axios.request({
	   url:"/backend/contactus",
	   method: "put",
	   data: data,
	 })
	   //	 console.log(backdata.data["Message"])
	 route.push("/welcome")
       }catch(err) {
	 console.log(err)
       }
     };
     return {
       data,
       sendusemail,
     }
   }
 });
</script>

<style scoped>

</style>
