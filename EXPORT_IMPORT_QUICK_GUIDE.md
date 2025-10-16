# Quick Guide: Mod Export/Import Feature

## What does it do?

The Export/Import feature allows you to:

1. **Export** your current mod list to a JSON file
2. **Import** that file on a fresh install to automatically re-download all your mods

## Step-by-Step Usage

### 📤 Exporting Your Mod List

1. Open CrossOver Mod Manager
2. Click on the **Settings** tab
3. Scroll down to "📦 Mod List Backup & Restore"
4. Click **"📤 Export Mod List"**
5. Choose where to save the file (e.g., Desktop, Documents)
6. Give it a name like `my-cyberpunk-mods.json`
7. Click **Save**

✅ Done! Your mod list is now saved.

### 📥 Importing Your Mod List

**Prerequisites:**

- You have a valid NexusMods API key configured in Settings
- You have the exported JSON file

**Steps:**

1. Open CrossOver Mod Manager (fresh install or existing)
2. Go to **Settings** → Configure your **NexusMods API Key** (if not already done)
3. Scroll down to "📦 Mod List Backup & Restore"
4. Click **"📥 Import Mod List"**
5. Select your exported JSON file
6. Click **"Yes"** to confirm
7. Switch to the **Logs** tab to watch progress

CMM will:

- ✓ Check which mods are already installed
- ✓ Skip mods that are already present
- 📥 Download and install missing mods
- 📊 Show you a summary when done

## Example Scenario

**Scenario:** You're upgrading to a new computer or reinstalling CrossOver

**Old Computer:**

1. Export your mod list → saves `cyberpunk-mods-backup.json`
2. Copy that file to cloud storage or USB drive

**New Computer:**

1. Install CrossOver Mod Manager
2. Configure Settings (game path, API key)
3. Import `cyberpunk-mods-backup.json`
4. ☕ Grab coffee while CMM re-downloads everything
5. ✅ All your mods are back!

## Important Notes

⚠️ **Only NexusMods mods can be exported/imported**

- Manually installed mods won't be included
- Mods need to have been downloaded through CMM's NXM protocol

🔑 **API Key is Required for Import**

- Get yours from: https://www.nexusmods.com/users/myaccount?tab=api
- Configure it in Settings before importing

📝 **Check the Logs Tab**

- Shows real-time progress during import
- Lists any errors or issues
- Confirms which mods were installed

## Troubleshooting

**"No mods available to export"**

- Make sure you have mods installed via NXM links
- Manually installed mods can't be exported

**"NexusMods API key is required"**

- Go to Settings and add your API key
- Get it from NexusMods account settings

**Some mods failed to download**

- Check if the mod is still available on NexusMods
- Some mods may have been deleted or hidden
- Check Logs tab for specific error messages

**Import is slow**

- Normal! Each mod needs to be downloaded from NexusMods
- Premium members get faster speeds
- Non-premium users may need to wait for download slots

## Tips

💡 **Regular Backups**: Export your mod list regularly, especially before major changes

💡 **Label Your Exports**: Use descriptive names like `cyberpunk-mods-2025-10-15.json`

💡 **Test First**: Try importing on a test setup to verify everything works

💡 **Keep API Key Handy**: Save your NexusMods API key somewhere safe for re-installs

## Need Help?

Check the **Logs** tab for detailed information about what's happening during export/import operations.
