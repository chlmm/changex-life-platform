/**
 * Hello World Plugin for Life Platform
 * This plugin demonstrates basic plugin functionality
 */

// Main function that will be called when the plugin is executed
async function main() {
  // Get plugin info
  const info = get_plugin_info();

  console.log(`Running ${info.name} v${info.version} by ${info.author}`);

  // List files in the plugin data directory
  const files = list_dir(".");

  if (files.length === 0) {
    // No files exist, create a greeting file
    write_file("greeting.txt", "Hello from Hello World Plugin!");
    console.log("Created greeting.txt");
  } else {
    // Read the greeting file
    const greeting = read_file("greeting.txt");
    console.log(`Greeting: ${greeting}`);
  }

  return "Hello World Plugin executed successfully!";
}

// Run the plugin
main();
