<script lang="ts">
    import Button from "@smui/button";
    import Fab, { Icon } from '@smui/fab';
    import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar';
    import IconButton from '@smui/icon-button';
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import {navigate} from 'svelte-navigator'
    import Tab, {Label } from '@smui/tab';
    import TabBar from '@smui/tab-bar';
    import MenuSurface from '@smui/menu-surface';
    import Textfield from '@smui/textfield';
    import Card, { Content } from '@smui/card';
    let name_person = "MyName";
    let privatekey = "745i475g6JY%R2734r6tgvy%23RC4JYU2UR"
    let privateseed = "blue eat ice cream niga eat banana my name niga who a you niga if yes okey"
    let loginpasswordpassword = "Andreyily20023"
    let loginpasswordlogin = "my_name_is_andrey_and_i_am_a_gey_2023"
    let loginpassword_or_privatekey = true
    const copy_my_text = (e: any) => {
      navigator.clipboard.writeText(e.target.innerText)
    }
</script>
<main>
    <h1>{name_person}</h1>
    <div class="card-container">
        <Card>
          <Content>PRIVATE KEY: <br><code on:click={copy_my_text}>{privatekey}</code></Content>
        </Card>
    </div>
    {#if loginpassword_or_privatekey == true}
      <div class="card-container">
        <Card>
          <Content>LOGIN: <br><code on:click={copy_my_text}>{loginpasswordlogin}</code>.</Content>
          <Content>PASSWORD: <br><code on:click={copy_my_text}>{loginpasswordpassword}</code>.</Content>
        </Card>
      </div>
    {:else}
      <div class="card-container">
        <Card>
          <Content on:click={copy_my_text}>PRIVATE SEED: <br><code>{privateseed}</code>.</Content>
        </Card>
      </div>
    {/if}
    <Button on:click={() => navigate("/main")}>Назад</Button> 
</main>
