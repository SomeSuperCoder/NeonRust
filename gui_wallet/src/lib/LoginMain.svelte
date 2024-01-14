<script lang="ts">
    import { Router, Route, Link } from "svelte-navigator";
</script>

<h1>Вход</h1>

<div class="option_buttons">
    <Link to="/login_pk" class="option__button">Приватный ключ</Link>
    <Link to="/login_seed" class="option__button">Сид фраза</Link>
    <Link to="/login_password" class="option__button">Логин и пароль</Link>
</div>

<style>
    .option_buttons {
        display: flex;
        flex-direction: column;
    }
</style>
