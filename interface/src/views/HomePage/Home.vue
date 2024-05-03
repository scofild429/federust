<template>
  <div class="container ps-0">
    <div class="row">
      <div class="col-3 col">
        <div class="col">
          <li class="mb-1">
            <button
              class="btn btn-success align-items-center  rounded-circle"
              data-bs-toggle="collapse"
              data-bs-target="#home-collapse1"
              aria-expanded="true"
            >
              <h3>Protocal</h3>
            </button>
	    <br/>
	    <br/>
            <div class="collapse show" id="home-collapse1">
              <ul class="btn-toggle-nav list-unstyled fw-normal pb-1 small">
                <li>
		    <router-link to="/products" class="link-dark rounded">
		    <button class="rounded-circle btn-success navbar-brand dropdown-item">
		      <h4>Start New </h4>
		    </button>
		  </router-link>
                </li>
                <li>
		  <router-link to="#" class="link-dark rounded">
		    <button class="rounded-circle btn-success navbar-brand dropdown-item">
		      <h4>Updates </h4>
		    </button>
		  </router-link>
		</li>
                <li>
		  <router-link to="#" class="link-dark rounded">
		    <button class="rounded-circle btn-success navbar-brand dropdown-item">
		      <h4>Reports </h4>
		    </button>
		  </router-link>
		</li>
              </ul>
            </div>
          </li>
        </div>
        <div class="col">
          <li class="mb-1">
            <button
              class="btn btn-success align-items-center rounded-circle"
              data-bs-toggle="collapse"
              data-bs-target="#home-collapse2"
              aria-expanded="true"
            >
              <h3>Inventory</h3>
            </button>
	    <br/>
	    <br/>
            <div class="collapse show" id="home-collapse2">
              <ul class="btn-toggle-nav list-unstyled fw-normal pb-1 small">
                <li>
		  <router-link to="/products" class="link-dark rounded">
		    <button class="rounded-circle btn-success navbar-brand dropdown-item">
		      <h4>Inventory1</h4>
		    </button>
		  </router-link>				
		</li>
		<router-link to="/products" class="link-dark rounded">
		  <button class="rounded-circle btn-success navbar-brand dropdown-item">
		    <h4>Inventory2 </h4>
		  </button>
		</router-link>				
              </ul>
            </div>
          </li>
        </div>
      </div>
      <div class="col-9 col">
        <div class="col">
          <img
	    src="../../assets/Welcome.png"
	    class="imgagebackground"
	  />
	  <div style="background: #61d1d1; box-shadow: 12px 12px 2px 1px rgba(0, 0, 255, .2);">
	  </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
 import {defineComponent} from "vue";
 
 export default defineComponent({
     name: "Home",
 });
</script>

<style scoped>
 .btn-toggle-nav a:hover,
 .btn-toggle-nav a:focus {
   background-color: #d2f4ea;
 }
 h5 {
   color: rgba(174, 100, 83, 0.7);
 }
 h4 {
   text-align: center;
 }
 blockquote {
   box-shadow: 0 0 0 1em #F4AAB9,
   0 0 0 1em #66CCFF;
   margin: 1em;
   padding:1em;
 }
 .imgagebackground {
   width: 100%;
   height: auto;
   max-width: 100%;
   display: block; 
 }
 
</style>
