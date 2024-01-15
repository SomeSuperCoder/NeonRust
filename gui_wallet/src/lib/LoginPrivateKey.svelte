<script lang="ts">
    import { Router, Route, Link } from "svelte-navigator";
    let seed_phrase = "";
    function generate_seed_phrase() {
        seed_phrase = "test"
    }

    function set_seed_pharase(e: any) {
        seed_phrase = e.target.value
    }
    function enter_login() {
        
    }
</script>

<div class="login_form">
    <h1>Вход по приватному ключу</h1>
    <textarea placeholder="Приватный ключ" bind:value={seed_phrase}></textarea> <!-- on:input={set_seed_pharase} value={seed_phrase} -->
    <button on:click={enter_login}></button>
    <button on:click={generate_seed_phrase}>Сгенерировать</button>
    <Link to="/">Назад</Link>
</div>

<style>
    .login_form {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .login_form * {
        width: 33vh;
    }

    textarea {
        height: 90px;
    }
</style>
