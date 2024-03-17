// @ts-nocheck
import './app.css'
import App from './App.svelte'
import init, {get_move} from '../public/pkg/connect4.js'

await init()

const app = new App({
  target: document.getElementById('app'),
  props: {
    get_move,
  }
})

export default app
