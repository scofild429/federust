<template>
  <div class="row">
    <div v-for="column in columnList" :key="column.id" class="col-4 mb-4">
      <div class="card h-100 shadow-sm">
        <div class="card-body text-center">
          <img
            :src="column.avatar"
            :alt="column.title"
            class="card-img-top rounded-circle border border-light w-25 my-3"
          />
          <h5 class="card-title">{{ column.title }}</h5>
          <p class="card-text text-left">{{ column.description }}</p>
          <router-link
            :to="`/column/${column.id}`"
            class="btn btn-outline-primary"
          >Go into Protocal PDF file</router-link>
        </div>
      </div>
    </div>
    <h1>
      <!-- <pre>{{ route }}</pre> -->
    </h1>
    <h1>{{ message }}</h1>
  </div>
</template>

<script lang="ts">
 import {defineComponent, computed, ref} from "vue";
 import {useStore} from "vuex";
 // import axios from "axios";
 
 interface ColumnProps {
   id: number;
   title: string;
   avatar: string;
   description: string;
 }

 export default defineComponent({
   name: "Protocol",
   setup() {
     const message = ref("");
     const store = useStore();
     const columnList = computed(() => {
       return store.state.Column.testData.map((column: ColumnProps) => {
         // if (!column.avatar) {
		 //   column.avatar = require("@/assets/logo.png");
		 // }
         return column;
       });
     });

     // onMounted(async () => {
	 // 	   const idToken = localStorage.getItem("idToken")
	 // 	   try {
	 // 		 const response = await axios.request({
	 // 		   url:"/backend/me",
	 // 		   method: 'get',
	 // 		   headers: {
	 // 			 Authorization: `Bearer ${idToken}`,
   // 		   }
	 // 		 })
	 // 		 // localStorage.setItem("username", response.data["name"]);
	 // 		 store.dispatch("login",  response.data["name"]);
	 // 	   }catch(err) {
	 // 		 console.log(err)
	 // 	   }
	 // 	 });
     return {
       columnList,
       message,
     };
   },
 });
</script>
