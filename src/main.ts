import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import VueQuillEditor from 'vue-quill-editor'
import VueMaterial from 'vue-material'

import 'vue-material/dist/vue-material.min.css'
import 'vue-material/dist/theme/default.css'

import 'quill/dist/quill.core.css' // import styles
import 'quill/dist/quill.snow.css' // for snow theme
import 'quill/dist/quill.bubble.css' // for bubble theme
Vue.use(VueMaterial)

Vue.use(VueQuillEditor, /* { default global options } */)
Vue.config.productionTip = false
document.title="Life of a simp"
new Vue({
	router,
	store,
	render: h => h(App)
}).$mount('#app')
