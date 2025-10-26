import { MasterChefBot } from "./bot";

/**
 * MasterChef Telegram Bot - Main Entry Point
 *
 * Usage:
 *   npm run telegram-bot
 *
 * Prerequisites:
 *   1. Create bot with @BotFather on Telegram
 *   2. Get bot token and add to .env as TELEGRAM_BOT_TOKEN
 *   3. Get your Telegram user ID and add to .env as TELEGRAM_ADMIN_IDS
 *   4. Ensure database has metrics (run collect-metrics first)
 */

async function main() {
  console.log("🚀 Starting MasterChef Telegram Bot...\n");

  try {
    const bot = new MasterChefBot();
    bot.start();

    console.log("✅ Bot is running! Send /start in Telegram to begin.\n");
    console.log("Press Ctrl+C to stop\n");

    // Handle graceful shutdown
    process.on("SIGINT", () => {
      console.log("\n\n📦 Shutting down gracefully...");
      bot.stop();
      process.exit(0);
    });

    process.on("SIGTERM", () => {
      console.log("\n\n📦 Shutting down gracefully...");
      bot.stop();
      process.exit(0);
    });
  } catch (error: any) {
    console.error("\n❌ Failed to start bot:");
    console.error(error.message);
    console.error("\nPlease check your configuration in .env file\n");
    process.exit(1);
  }
}

main().catch((error) => {
  console.error("❌ Fatal error:");
  console.error(error);
  process.exit(1);
});
