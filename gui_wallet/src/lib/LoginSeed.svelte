<script lang="ts">
    import { Router, Route, Link, navigate } from "svelte-navigator";
    import Textfield from '@smui/textfield';
    import Button, { Label } from '@smui/button';

    let seed_phrase = "";
    function generate_seed_phrase() {
        seed_phrase = "test"
    }

    function set_seed_pharase(e: any) {
        seed_phrase = e.target.value
    }
    function enter_main(){
        navigate("/main")
    }
</script>

<div class="login_form">
    <h1>Вход по сид фразе</h1>
    <Textfield label="Приватный ключ" bind:value={seed_phrase} textarea></Textfield> <!-- on:input={set_seed_pharase} value={seed_phrase} -->
    <!-- on:input={set_seed_pharase} value={seed_phrase} -->
    <Button on:click={generate_seed_phrase}>Сгенерировать</Button>
    <Button variant="raised" on:click={enter_main}>Войти</Button>
    <Button on:click={() => navigate("/")}>
        <Label>Назад</Label>
    </Button>
</div>

<style>
    /* :root {
        min-height: 100vh;
        place-items: center;
        display: flex;
        margin: 0 auto;
    } */
    .login_form {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    /* 
    .login_form * {
        width: 33vh;
    } */
/* 
    textarea {
        height: 90px;
    } */
</style>
