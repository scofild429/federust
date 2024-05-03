<template>
  <div class="about">
    <h3 class="boxxx">welcome to  update  or company </h3>
    <div class="container inputbox">
      <div class="row">
	<div class="col-md-2 offset-md-2"  style="text-align: right">User Email</div>
	<div class="col-md-5">
	  <input
	    v-model="email"
	    style="width: 100%"
	    type="email"
	    class="form-control boxxx"
	    placeholder="Email"
	    required
	  />
	</div>
      </div>
    </div>
    <div class="container inputbox">
      <div class="row">
	<div class="col-md-2 offset-md-2" style="text-align: right">User Name</div>
	<div class="col-md-5">
	  <input
	    v-model="name"
	    style="width: 100%"
	    type="text"
	    class="form-control boxxx"
	    placeholder="Name"
	    required
	  />
	</div>
      </div>
    </div>
    <div class="container inputbox">
      <div class="row">
	<div class="col-md-2 offset-md-2" style="text-align: right">Send email to </div>
	<div class="col-md-5">
	  <input
	    v-model="data.to_email"
	    style="width: 100%"
	    type="text"
	    class="form-control boxxx"
	    placeholder="the email address of staff person"
	    required
	  />
	</div>
      </div>
    </div>
    <div class="container inputbox">
      <div class="row">
	<div class="col-md-2 offset-md-2" style="text-align: right">Contact Person</div>
	<div class="col-md-5">
	  <input
	    v-model="data.contact_person"
	    style="width: 100%"
	    type="text"
	    class="form-control boxxx"
	    placeholder="the name of staff person"
	    required
	  />
	</div>
      </div>
    </div>
    <div class="container inputbox">
      <div class="row">
	<div class="col-md-2 offset-md-2" style="text-align: right">Use system email </div>
	<div class="form-check col-md-5 ">
	  <input class="form-check-input" type="checkbox" value="is" v-model="usesystememail">
	  <label class="form-check-label" for="flexCheckDefault">
	    you  may not able to review your contact meassage
	  </label>
	</div>
      </div>
    </div>
    <div class="container inputbox">
      <div class="row">
	<div class="col-md-2 offset-md-2" style="text-align: right">Contact email</div>
	<div class="col-md-5">
	  <input
	    v-model="data.from_email"
	    :readonly=usesystememail
	    style="width: 100%"
	    type="text"
	    class="form-control boxxx"
	    placeholder="Only support outlook mail for now)"
	    required
	  />
	</div>
      </div>
    </div>
    <div class="container inputbox">
      <div class="row">
	<div class="col-md-2 offset-md-2" style="text-align: right">Email password</div>
	<div class="col-md-5">
	  <input
	    v-model="data.from_email_password"
	    :readonly=usesystememail
	    style="width: 100%"
	    type="password"
	    class="form-control boxxx"
	    placeholder="Password for your contact email, you don't have to fill"
	    required
	  />
	</div>
      </div>
    </div>
    <button class="w-10 btn-lg boxxx btn-primary" @click.prevent="submitreflash">Submit</button>
  </div>
</template>

<script lang="ts">
 import {defineComponent, reactive,ref, watch, onBeforeMount, computed } from "vue";
 import axios from "axios";
 import {useStore} from "vuex";
 import {useRouter} from "vue-router";

 export default defineComponent({
   name: "Profile",
   setup() {
     const store = useStore();
     const router = useRouter();
     const idToken = localStorage.getItem("idToken");
     const usesystememail = ref(false);
     const email = computed(() => store.state.user.email);
     const profileId = computed(() => store.state.user.profileId);
     const name  = computed(() => store.state.user.name)

     const data = reactive({
       contact_person:"",
       to_email: "",
       from_email: "",
       from_email_password:"",
     });
     

     ///////////////////////////////////////////
     //
     // get all items from database and show them at first
     //
     ///////////////////////////////////////////
     onBeforeMount(() => {sendtoDBsubmitrevievial()});
     const sendtoDBsubmitrevievial =  async () =>{
       try {
	 const dataResp = await fetch(import.meta.env.VITE_Domain+"/api/profile", {
	   method: "GET",
	   credentials: 'include',
	 });
	 const json_data = await dataResp.json();
	 data.contact_person = json_data["contact_person"]
	 data.to_email = json_data["to_email"]
	 data.from_email = json_data["from_email"]
	 data.from_email_password = json_data["from_email_password"]
       }catch(err) {
 	 console.log(err)
       }
     };

     watch(usesystememail, ()=>{
       if(usesystememail.value == true){
	 data.from_email = "q3labs@outlook.com";
       }
       else {
	 data.from_email = "";
	 data.from_email_password = "";
       }
     });
     
     const submitreflash = async () => {
       try {
	 const dataResp = await fetch(import.meta.env.VITE_Domain+"/api/profile/", {
	   method: "PUT",
	   credentials: 'include',
	   headers: { "Content-Type": "application/json" },
	   body: JSON.stringify(data)
	 });
	 router.push("/");
       }catch(err) {
	 console.log(err)
       }
     };
     return {
       name,
       email,
       data,
       submitreflash,
       usesystememail
     }
   }
 })
</script>

<style scoped>
 .boxxx {
   margin: 0 auto;
   display: table;
 }
 .inputbox {
   margin: 20px;
 }
 
</style>
