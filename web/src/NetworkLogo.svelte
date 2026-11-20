<script lang="ts">
  import { siInstagram, siTelegram, siWhatsapp, siX } from "simple-icons";
  import { siLinkedin, siMicrosoftteams } from "simple-icons-legacy";
  import type { Network } from "./accounts";

  let { network, size = 16 }: { network: Network; size?: number } = $props();

  type Mark = { title: string; hex: string; path: string };
  const icons: Record<Network, Mark> = {
    WhatsApp: siWhatsapp,
    Telegram: siTelegram,
    Instagram: siInstagram,
    LinkedIn: siLinkedin,
    X: siX,
    Teams: siMicrosoftteams,
  };
  const icon = $derived(icons[network]);
</script>

<svg width={size} height={size} viewBox="0 0 24 24" role="img" aria-label={icon.title} fill="#{icon.hex}">
  <path d={icon.path} />
</svg>
