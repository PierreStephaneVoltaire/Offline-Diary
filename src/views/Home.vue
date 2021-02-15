<template>
  <div class="home">
    <Nav></Nav>
    <div>
      <MdButton class="md-raised md-success md-primary" @click="goToContent"><span>Go forth and write which can not be spoken to anyone</span>
        <md-icon class="dark">chevron_right</md-icon>
      </MdButton>
    </div>
  </div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import HelloWorld from '@/components/HelloWorld.vue';
import Nav from "@/components/Nav.vue";

@Component({
  components: {
    Nav,
    HelloWorld,
  },
})
export default class Home extends Vue {
  location = "entry"
  goToContent = () => {
    this.$router.push("/entry")

  }
}
</script>
<style scoped>


</style>