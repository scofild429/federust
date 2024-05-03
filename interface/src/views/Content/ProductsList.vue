<template>
    <div class="container ps-0">
	<div class="row">
	    <div class="col-3 col">
		<div class="col">
		    <router-link to="#">
			<button class="rounded-circle btn-success navbar-brand dropdown-item">
			    <h4>Commercial</h4>
			</button>
		    </router-link>
		    <router-link to="#">
			<button class="rounded-circle btn-success navbar-brand dropdown-item">
			    <h4>Open Project</h4>
			</button>
		    </router-link>
		    <router-link to="#">
			<button class="rounded-circle btn-success navbar-brand dropdown-item">
			    <h4>Under developing</h4>
			</button>
		    </router-link>
		    <router-link to="#">
			<button class="rounded-circle btn-success navbar-brand dropdown-item">
			    <h4>Self design</h4>
			</button>
		    </router-link>
		</div>
	    </div>
	    <div class="col-9 col">
		<div class="col">
		    <div>
			<h2>Welcome to Q3Labs digtial service</h2>
			<h5>
			    There are following things should to be considered before your
			    start
			</h5>
			<br />
			<br />
			<br />
			<div class="container overflow-hidden">
			    <div class="row gy-5">
				<div class="col-6">
				    <div class="p-3 border bg-light">Check your kids</div>
				</div>
				<div class="col-6">
				    <div class="p-3 border bg-light">Read the Manule</div>
				</div>
				<div class="col-6">
				    <div class="p-3 border bg-light">Hygienic condition</div>
				</div>
				<div class="col-6">
				    <div class="p-3 border bg-light">Special check(Is it poisonours?)</div>
				</div>
			    </div>
			</div>
			<br />
			<br />
			<h5>please choose your experiment to start</h5>
			<div class="row gy-5">
			    <div style="display: flex; flex-direction: column">
				<select id="sendSybol" v-model="selected">
				    <option v-for="pro in products" :key="pro.id" :value="pro.id">{{ pro.name }}</option>
				</select>
			    </div>
			</div>
			<br />
			<button class="w-10 btn-lg boxxx btn-success" @click.prevent="GotoContent">Start</button>
		    </div>
		</div>
	    </div>
	</div>
    </div>
</template>

<script lang="ts">
 import { defineComponent, ref } from "vue";
 import { useRouter } from "vue-router";

 export default defineComponent({
     name: "Product",
     data() {
	 return {
	     products: [
		 { id: 1, name: "Preparation of the tissue lysate" },
		 { id: 2, name: "Immunocytochemistry" },
		 { id: 3, name: "Cell lysate preparation for protein analysis" },
		 { id: 4, name: "Production of lentiviral particles for transduction​" },
		 { id: 5, name: "lentiviral transduction" },
		 { id: 6, name: "Dr. Prasad Dasari​ Project" },
		 { id: 7, name: "Experiment 07" },
	     ],
	 };
     },
     setup() {
	 const router = useRouter();
	 const selected = ref(0);
	 const GotoContent = () => {
	     switch (selected.value) {
		 case 1:
		     router.push("/content");
		     break;
		 case 2:
		     router.push("/session");
		     break;
		 case 3:
		     router.push("/session03/stepInit");
		     break;
		 case 4:
		     router.push("/session04/stepParaReagents");
		     break;
		 case 5:
		     router.push("/session05/stepInit");
		     break;
		 case 6:
		     router.push("/session06/step01");
		     break;
		 default:
	     }
	 };
	 return {
	     GotoContent,
	     selected,
	 };
     },
 });
</script>

<style scoped>
 h4 {
     color: rgba(17, 55, 94, 0.8);
 }
 h4 {
     text-align: center;
 }
 .boxxx {
     margin: 0 auto;
     display: table;
 }
</style>
