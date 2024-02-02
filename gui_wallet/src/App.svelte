<script lang="ts">
  import LoginMain from './lib/LoginMain.svelte';
  import { Router, Route, Link } from "svelte-navigator";
  import LoginPrivateKey from "./lib/LoginPrivateKey.svelte";
  import LoginSeed from "./lib/LoginSeed.svelte";
  import LoginPassword from "./lib/LoginLoginPassword.svelte"
  import { invoke } from '@tauri-apps/api'
  import MainPage from "./lib/MainPage.svelte"
  import Send from './lib/Send.svelte';
  import Person from './lib/Person.svelte';
  import TransactionWait from './lib/TransactionWait.svelte';
  // invoke('greet', { name: 'World' }).then((response) => console.log(response))
</script>

<main>
  <Router>
    <Route path="/">
      <LoginMain />
		</Route>
    <Route path="/login_pk">
      <LoginPrivateKey />
		</Route>
    <Route path="/login_seed">
      <LoginSeed />
    </Route>
    <Route path="/login_password">
      <LoginPassword />
    </Route>
    <Route path="/main">
      <MainPage />
    </Route>
    <Route path="/send">
      <Send />
    </Route>
    <Route path="/person">
      <Person />
    </Route>
    <Route path="/transaction">
      <TransactionWait />
    </Route>
  </Router>
</main>
