<script lang="ts">
    import { Router, Route, Link, navigate } from "svelte-navigator";
    import Button, { Label } from "@smui/button";
</script>

<main class="login">
    <div class="option_buttons">
        <h2>Вход</h2>
        <Button variant="raised" on:click={() => navigate("/login_pk")}>
            <Label>Приватный ключ</Label>
        </Button>
        <br>
        <Button variant="raised" on:click={() => navigate("/login_seed")}>
            <Label>Сид фраза</Label>
        </Button>
        <br>
        <Button variant="raised" on:click={() => navigate("/login_password")}>
            <Label>Логин и пароль</Label>
        </Button>
    </div>
</main>
<style>
    /* .option_buttons {
        display: flex;
        flex-direction: column;
        flex-grow: ;
    } */
    main.login {
        display: flex;
        place-items: center;
        justify-content: center;
        /* min-width: 320px; */
        min-height: 100vh;
    }
</style>
