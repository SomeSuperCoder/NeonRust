<script lang="ts">
    import Button from "@smui/button";
    import Fab, { Icon } from '@smui/fab';
    import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar';
    import IconButton from '@smui/icon-button';
    import Checkbox from '@smui/checkbox';
    import FormField from '@smui/form-field';
    import {navigate} from 'svelte-navigator'
    let user_addres = "0xABOBA";
    import Tab, {Label } from '@smui/tab';
    import TabBar from '@smui/tab-bar';
  
    let tabs = [
      {
        icon: '$',
        label: 'Токены',
      },
      {
        icon: 'image',
        label: 'NFT',
      },
      {
        icon: 'access_time',
        label: 'История',
      },
    ];
    let active = tabs[0];
    let balance_number = 0;
</script>

<main>
    <TopAppBar variant="static">
      <Row>
        <Section>
          <IconButton class="material-icons">menu</IconButton>
          <Title>0xABOBA</Title>
        </Section>
        <Section align="end" toolbar>
          <IconButton class="material-icons" aria-label="Download">person</IconButton>
          <IconButton class="material-icons" aria-label="Print this page">settings</IconButton>
        </Section>
      </Row>
    </TopAppBar>
    <div class="neon_part">
        <h1 class="balance"><span class="balance_number">{balance_number}</span> NEON</h1>
        <Fab color="primary" on:click={() => navigate("/send")}>
            <Icon class="material-icons">send</Icon>
        </Fab>
        <Fab color="primary" on:click={() => navigate("https://example.com")}>
          <Icon class="material-icons">lock</Icon>
      </Fab>
    </div>
    <div>
      <TabBar {tabs} let:tab bind:active>
        <Tab {tab}>
          <Icon class="material-icons">{tab.icon}</Icon>
          <Label>{tab.label}</Label>
        </Tab>
      </TabBar>
    </div>
</main>
