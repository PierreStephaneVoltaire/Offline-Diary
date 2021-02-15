<template>
  <div id="nav-section">
    <img id="site-logo" alt="Vue logo" src="../assets/logo.jpg">
    <h1>{{ siteTitle }}</h1>
    <h3>“There is no greater agony than bearing an untold story inside you.” — Maya Angelou</h3>
    <div class="nav">
      <router-link class="link" to="/">Login</router-link>
      |
      <router-link class="link" to="/about">About</router-link>
      |
      <router-link class="link" to="/settings">Settings</router-link>
      |
      <router-link class="link" to="/nicknames">King and Queens</router-link>
    </div>
    <div class="nav">
      <router-link class="link" to="/Mood">Mood</router-link>
      |
      <router-link class="link" to="/simping">Simp Stats</router-link>
    </div>
  </div>
</template>

<script>
import {Vue} from "vue-property-decorator";

export default class Nav extends Vue {
  name= "Nav";
  siteTitle = "I simp therefore i am!"
}

</script>

<style scoped>
#site-logo {
  height: 200px;
  width: auto;
  border: 1px solid lightgray;
  border-radius: 50%;
}

#nav-section {
  font-weight: bold;
}

.nav {
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin-bottom: 15px;
}

.nav .link {
  padding-left: 30px;
  padding-right: 30px;

}
</style>