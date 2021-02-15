<template>
  <div id="app">
    <md-app>


      <md-app-content>
        <router-view></router-view>
      </md-app-content>
    </md-app>
  </div>
</template>
<style lang="scss">
@import url('https://fonts.googleapis.com/css?family=Fira+Code');
html, body {
  font-family: 'Fira Code', sans-serif !important;
}

#app {
  font-family: 'Fira Code', sans-serif !important;

  // node-sass 4.8+ broke md-material
  --md-theme-default-primary: #42b983;
  --md-theme-default-primary-on-background: #42b983;
}

#app {

  height: 100vh;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #A7EDD4;
}

.md-success {
  font-weight: bold !important;
  color: #2c3e50 !important;

}

.dark {
  color: #2c3e50 !important;
}

body {
  background: white;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}


.ql-container.ql-snow {
  height: auto;
}
.ql-editor {
  max-height: 100% !important;
  display: flex;
  flex-direction: column;

  flex-shrink: 0;

}
.quill-editor{
  max-height: 100% !important;
  display: flex !important;
  flex-direction: column !important;

  flex-shrink: 0;


}
.ql-container.ql-snow {
border: none !important;
max-height: 100%;
overflow: auto;
}



</style>
<script>
import Nav from "@/components/Nav";

export default {
  components: {}
}
</script>