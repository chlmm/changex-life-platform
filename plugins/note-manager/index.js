/**
 * Note Manager Plugin for Life Platform
 * A simple note taking application
 */

var NOTES_DIR = "notes";

// Initialize plugin
function init() {
  console.log("Note Manager Plugin v1.0.0");
  console.log("Commands: list, create <title>, read <title>, delete <title>");
  console.log("========================================");
  
  // Ensure notes directory exists
  ensureNotesDir();
}

// Ensure notes directory exists
function ensureNotesDir() {
  try {
    var files = list_dir(".");
    var hasDir = false;
    for (var i = 0; i < files.length; i++) {
      if (files[i] === NOTES_DIR) {
        hasDir = true;
        break;
      }
    }
    if (!hasDir) {
      // Create directory by writing a dummy file
      write_file(NOTES_DIR + "/.keep", "");
      console.log("Created notes directory");
    }
  } catch (e) {
    console.log("Could not create notes directory: " + e);
  }
}

// List all notes
function listNotes() {
  console.log("");
  console.log("Your Notes:");
  console.log("----------------------------------------");
  
  try {
    var files = list_dir(NOTES_DIR);
    // Use simple loop instead of filter
    var notes = [];
    for (var i = 0; i < files.length; i++) {
      if (files[i].indexOf('.txt') !== -1) {
        notes.push(files[i]);
      }
    }
    
    if (notes.length === 0) {
      console.log("  No notes yet. Create one with: create <title>");
    } else {
      for (var j = 0; j < notes.length; j++) {
        var title = notes[j].replace('.txt', '');
        console.log("  " + (j + 1) + ". " + title);
      }
    }
    
    console.log("");
    console.log("  Total: " + notes.length + " note(s)");
  } catch (e) {
    console.log("  Error listing notes: " + e);
  }
}

// Create a new note
function createNote(title, content) {
  if (!title) {
    console.log("Error: Title is required");
    return;
  }
  
  var filename = NOTES_DIR + "/" + title + ".txt";
  var noteContent = content || "Note created\n\n";
  
  try {
    write_file(filename, noteContent);
    console.log("Created note: \"" + title + "\"");
  } catch (e) {
    console.log("Error creating note: " + e);
  }
}

// Read a note
function readNote(title) {
  if (!title) {
    console.log("Error: Title is required");
    return;
  }
  
  var filename = NOTES_DIR + "/" + title + ".txt";
  
  try {
    var content = read_file(filename);
    console.log("");
    console.log(title + ":");
    console.log("----------------------------------------");
    console.log(content);
    console.log("----------------------------------------");
  } catch (e) {
    console.log("Note \"" + title + "\" not found");
  }
}

// Delete a note
function deleteNote(title) {
  if (!title) {
    console.log("Error: Title is required");
    return;
  }
  
  var filename = NOTES_DIR + "/" + title + ".txt";
  
  try {
    // Write empty to delete (simulated delete)
    write_file(filename, "");
    console.log("Deleted note: \"" + title + "\"");
  } catch (e) {
    console.log("Error deleting note: " + e);
  }
}

// Demo - create some sample notes
function createDemoNotes() {
  createNote("Welcome", "Welcome to Note Manager!\n\nThis is a simple note taking plugin.\n\nFeatures:\n- Create notes\n- Read notes\n- List all notes\n- Delete notes");
  
  createNote("TODO", "TODO List:\n\n1. Learn Life Platform plugin system\n2. Create awesome plugins\n3. Share with friends");
  
  createNote("Ideas", "Plugin Ideas:\n\n- Calculator\n- Weather widget\n- File manager\n- Task tracker");
}

// Main function
function main() {
  init();
  
  // Create demo notes if directory is empty
  try {
    var files = list_dir(NOTES_DIR);
    // Count .txt files
    var noteCount = 0;
    for (var i = 0; i < files.length; i++) {
      if (files[i].indexOf('.txt') !== -1 && files[i] !== '.txt') {
        noteCount = noteCount + 1;
      }
    }
    
    if (noteCount === 0) {
      console.log("");
      console.log("Creating demo notes...");
      createDemoNotes();
    }
  } catch (e) {
    // Directory might not exist yet
  }
  
  // Show all notes
  listNotes();
  
  console.log("");
  console.log("Try these commands in your plugin code:");
  console.log("  createNote('My Note', 'Content here...')");
  console.log("  readNote('My Note')");
  console.log("  listNotes()");
  console.log("  deleteNote('My Note')");
  
  return "Note Manager Plugin executed successfully!";
}

// Run the plugin
main();
