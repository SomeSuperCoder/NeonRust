<script lang="ts">
    import CircularProgress from '@smui/circular-progress'
    import Accordion, { Panel, Header, Content } from '@smui-extra/accordion'
    import IconButton, { Icon } from '@smui/icon-button'
    
    let amount_spent = 123.34
    let panel1Open = false
    let upload = true
    let success = true
    let icon = 'https://media.tenor.com/dHcBEGzvgDsAAAAi/emoji-emojis.gif'
</script>
{#if upload == false}
    <div style="display: flex; justify-content: center">
        <CircularProgress style="height: 80px; width: 80px;" indeterminate />
    </div>
    <h1>Ожидание...</h1>
{:else}
    {#if success == true} 
        <div>
            <Icon class="material-icons">star</Icon>
            <h1 id="ready" style="font-size: 50px;">Упешно</h1>
            <Icon class="material-icons">star</Icon>
        </div> 
        <h2 class="color_red">- {amount_spent} NEON</h2>
    {/if} 
    {#if success != true}
        <Icon class="material-icons">error</Icon>
        <h1 id="ready" class="color_red">Ошибка</h1>
        <Icon class="material-icons">error</Icon>
    {/if}
    <!-- Текст с результатом -->
    <!-- Кнопка логи -->
    {#if deplover_mod}   
        <div class="accordion-container">
            <Accordion>
                <Panel bind:open={panel1Open}>
                    <Header>
                        Для разработчиков
                        
                    </Header>
                    <Content>
                        === SystemProgram start ===<br>
                        All accounts retreived<br>
                        Sending 100 Neon from Alice to Bob<br>
                        !! -- Error -- !!<br>
                        === Error niga ===
                    </Content>
                </Panel>
            </Accordion>
        </div>
    {/if}
{/if}

<style>
    #done_icon {
        display: inline;
        width: 100px;
        height: 100px;
    }

    h1#ready {
        display: inline;
    }
    .color_red{
        color: red;
    }
</style>
