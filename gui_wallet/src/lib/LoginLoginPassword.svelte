<script lang="ts">
    import { Router, Route, Link } from "svelte-navigator";
</script>

<div class="login_form">
    <h1>Вход по логину и паролю</h1>
    <input type="text" placeholder="Логин">
    <input type="password" placeholder="Пароль">
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
</style>
