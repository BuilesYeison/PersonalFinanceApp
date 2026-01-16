<script lang="ts">
  import {
    ArrowLeftRightIcon,
    BikeIcon,
    BookIcon,
    SettingsIcon,
    HouseIcon,
    TreePalmIcon,
  } from "lucide-svelte";
  import { Navigation } from "@skeletonlabs/skeleton-svelte";
  let { children } = $props();
  const links = [
    { label: "Home", href: "/#", icon: HouseIcon },
    { label: "Entertainment", href: "/#", icon: BookIcon },
    { label: "Recreation", href: "/#", icon: BikeIcon },
    { label: "Relaxation", href: "/#", icon: TreePalmIcon },
  ];

  let layoutRail = $state(true);

  function toggleLayout() {
    layoutRail = !layoutRail;
  }
</script>

<div
  class="w-full h-screen grid grid-cols-[auto_1fr] items-stretch border border-surface-200-800"
>
  <!-- --- -->
  <Navigation
    layout={layoutRail ? "rail" : "sidebar"}
    class={layoutRail ? "" : "grid grid-rows-[1fr_auto] gap-4"}
  >
    <Navigation.Content>
      <Navigation.Header>
        <Navigation.Trigger onclick={toggleLayout}>
          <ArrowLeftRightIcon class={layoutRail ? "size-5" : "size-4"} />
          {#if !layoutRail}<span>Resize</span>{/if}
        </Navigation.Trigger>
      </Navigation.Header>
      <Navigation.Menu>
        {#each links as link (link)}
          {@const Icon = link.icon}
          <Navigation.TriggerAnchor>
            <Icon class={layoutRail ? "size-5" : "size-4"} />
            <Navigation.TriggerText>{link.label}</Navigation.TriggerText>
          </Navigation.TriggerAnchor>
        {/each}
      </Navigation.Menu>
    </Navigation.Content>
    <Navigation.Footer>
      <Navigation.TriggerAnchor href="/" title="Settings" aria-label="Settings">
        <SettingsIcon class="size-4" />
        <Navigation.TriggerText>Settings</Navigation.TriggerText>
      </Navigation.TriggerAnchor>
    </Navigation.Footer>
  </Navigation>
  <!-- --- -->
  <div class="p-2">
    {@render children()}
  </div>
</div>
