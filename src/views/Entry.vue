<template>
  <div id="entry">
    <div class="first">
      <div class="bread">
        <router-link class="link" to="/">Home</router-link>
        <MdIcon>chevron_right</MdIcon>
        <router-link class="link" to="/entry">Entries</router-link>
        <MdIcon>chevron_right</MdIcon>
        <router-link class="link" to="/entry">{{ selectedDate.toDateString() }}</router-link>
      </div>
      <div class="arrow-section">
      <MdInput maxlength="20" placeholder="Main topic..." class="md-title custom-title">jj</MdInput>


    </div>
      <div class="date-section">
        <md-datepicker class="date-picker" :md-open-on-focus="false" v-model="selectedDate"/>
      </div>
    </div>

    <div id="editor-container">



      <div></div>
      <quill-editor :disabled="false" v-model="content"
                    ref="myQuillEditor"
                    :options="editorOption">

      </quill-editor>
    </div>
  </div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import Nav from "@/components/Nav.vue";
import format from 'date-fns/format'

@Component({
  components: {
    Nav,
  },
})

export default class Entry extends Vue {
  content = "Life of a simp";
  now = new Date();

  get selectedDate() {
    return this.now
  }

  set selectedDate(val) {
    this.now = val
  }

  editorOption: {} = {
    modules: {
      toolbar: [['bold', 'italic', 'underline', 'strike'], ['blockquote', 'code-block'], [{'list': 'ordered'}, {'list': 'bullet'}], [{'indent': '-1'}, {'indent': '+1'}]
        , [{'header': [1, 2, 3, 4, 5, 6, false]}]
        , ['link', 'image', 'video']],

    },
    debug: 'info',
    placeholder: 'Compose an epic...',
    readOnly: true,
    theme: 'snow'
  }
}
</script>
<style scoped>
.first{    min-height: 33%;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;}
#entry{
  height: calc(100vh - 32px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  justify-content: space-between;
}
.bread {
  display: flex;
  flex-direction: row;
  font-weight: bold;
  justify-content: flex-start;
  margin-bottom: 50px;
}

.bread .md-icon {
  margin-top: auto;
  margin-bottom: auto;
  margin-left: 10px !important;
  margin-right: 10px !important;
}

.custom-title {
  font-size: 1.5rem;
  margin-bottom: 50px;
  text-align: center;
  border: none;
}

.date-section {
  display: flex;
  flex-direction: row-reverse;
  padding-left: 50px;
  padding-right: 50px;
}

.date-picker {
  width: unset !important;
}

.arrow-section {
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin-bottom: 50px;
  padding-left: 50px;
  padding-right: 50px;

}

#editor-container {
  overflow: hidden;
  max-height: calc(66% - 82px);
  height: 100%;
  display: flex;
  flex-direction: column;
  background: white;
}


</style>