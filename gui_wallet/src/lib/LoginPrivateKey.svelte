<script lang="ts">
    import { Router, Route, Link, navigate } from "svelte-navigator";
    import Textfield from '@smui/textfield';
    import Button, { Label } from '@smui/button';

    let seed_phrase = "";
    let inputValue = 'Write here!';
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
    <h1>Вход по приватному ключу</h1>
    <Textfield label="Приватный ключ" bind:value={seed_phrase} textarea></Textfield> <!-- on:input={set_seed_pharase} value={seed_phrase} -->
    <Button on:click={generate_seed_phrase}>
        <Label>Сгенерировать</Label>
    </Button>
    <Button variant="raised" on:click={enter_main}>Войти</Button>
    <Button on:click={() => navigate("/")}>
        <Label>Назад</Label>
    </Button>
</div>

<style>
    .login_form {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    textarea {
        height: 90px;
    }
</style>
