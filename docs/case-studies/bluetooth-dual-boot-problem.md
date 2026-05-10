# Bluetooth Dual-Boot Pairing Problem

## Summary

Bluetooth devices can fail to reconnect on Linux after being used on Windows in a dual-boot setup. The device may still appear as paired, bonded, and trusted in Linux, but connection attempts hang, fail, or only work after re-pairing.

This happens because Windows and Linux store Bluetooth pairing credentials separately. If both operating systems use the same physical Bluetooth adapter, the peripheral sees them as the same host identity. When Windows pairs or updates the bond, Linux can be left with stale keys.

The practical fix is to sync the Bluetooth bond keys from Windows into Linux BlueZ.

## Observed Case

Machine:

- Laptop: Lenovo Legion
- Linux Bluetooth adapter: `F8:89:D2:83:92:C0`
- Mouse: Legion M600 Mouse
- Linux original mouse identity: `C6:C0:FC:F1:FB:80`
- Windows-current mouse identity: `C6:C0:FD:F1:FB:80`

Symptoms:

- If the mouse was used only on Linux, it auto-connected after reboot.
- If the mouse was used on Windows, then booted back into Linux, it would not reconnect reliably.
- Linux still showed:

```text
Paired: yes
Bonded: yes
Trusted: yes
Blocked: no
Connected: no
```

Earlier power-management fixes were already in place and verified:

```text
btusb enable_autosuspend=N
power/control: on
power/wakeup: enabled
```

So the recurring issue was not USB autosuspend. It was stale Bluetooth bonding state.

## Why It Happens

Bluetooth pairing is not just a saved device name. For BLE devices, the host and peripheral exchange long-term security material. Important values include:

- `LTK`: Long Term Key, used for encrypted reconnection.
- `IRK`: Identity Resolving Key, used to resolve private or changing BLE identities.
- `CSRK`: Connection Signature Resolving Key, used for signed data.
- `EDIV` and `Rand`: metadata used with the long-term key.

Linux BlueZ stores these keys under:

```text
/var/lib/bluetooth/<adapter-address>/<device-address>/info
```

Windows stores its Bluetooth keys in the SYSTEM registry hive, under:

```text
SYSTEM\ControlSet00N\Services\BTHPORT\Parameters\Keys\<adapter-address>
```

In this case, Windows had a newer bond for the M600 than Linux. The mouse accepted Windows' current key, while Linux kept trying to authenticate using an older key. From Linux, the device still looked paired, but the actual cryptographic relationship no longer matched.

## Why Re-Pairing Is Not Ideal

Re-pairing in Linux can make Linux work again, but it may break Windows next. Re-pairing in Windows can make Windows work again, but it may break Linux next.

That is the loop dual-boot users commonly hit:

```text
Pair in Linux -> Windows has stale keys
Pair in Windows -> Linux has stale keys
Repeat
```

The correct solution is to make both operating systems use the same bond keys.

## Manual Fix Model

The successful approach was:

1. Mount the Windows partition read-only.
2. Read the offline Windows SYSTEM registry hive.
3. Locate the active Windows control set.
4. Find Bluetooth keys under `BTHPORT\Parameters\Keys`.
5. Identify the Bluetooth adapter address.
6. Identify the target peripheral by name, VID/PID, address, and recent connection time.
7. Convert the Windows key values into BlueZ `info` format.
8. Back up Linux's existing BlueZ records.
9. Stop `bluetooth.service`.
10. Write or update the matching BlueZ device record.
11. Start `bluetooth.service`.
12. Connect and verify using `bluetoothctl`.

## What We Actually Did

On the Legion M600 case, the implemented fix followed this concrete sequence:

1. Verified Linux still knew the mouse:

```text
Device C6:C0:FC:F1:FB:80
Paired: yes
Bonded: yes
Trusted: yes
Blocked: no
Connected: no
```

2. Verified the previous Bluetooth power-management fix was still active:

```text
btusb enable_autosuspend=N
power/control: on
power/wakeup: enabled
```

This ruled out USB autosuspend as the current cause.

3. Confirmed the Windows partition was mounted read-only:

```text
/mnt/windows
```

4. Installed registry hive tools on Linux:

```bash
sudo apt-get install -y chntpw libhivex-bin
```

5. Read the offline Windows SYSTEM hive:

```text
/mnt/windows/Windows/System32/config/SYSTEM
```

6. Found the active Windows control set:

```text
ControlSet001
```

7. Found the Windows Bluetooth adapter key:

```text
SYSTEM\ControlSet001\Services\BTHPORT\Parameters\Keys\f889d28392c0
```

This corresponds to the Linux adapter:

```text
F8:89:D2:83:92:C0
```

8. Found multiple Windows-side Legion M600 identities:

```text
C6:C0:FD:F1:FB:80
C6:C0:FA:F1:FB:80
C6:C0:F8:F1:FB:80
```

9. Compared Windows `LastConnected` timestamps and selected the newest identity:

```text
C6:C0:FD:F1:FB:80
```

10. Extracted the Windows bond values for that identity:

```text
LTK
IRK
CSRK
ERand
EDIV
Address
AddressType
```

11. Converted the Windows values into BlueZ `info` format:

```text
IRK  -> [IdentityResolvingKey] Key
CSRK -> [LocalSignatureKey] Key
LTK  -> [LongTermKey] Key
EDIV -> [LongTermKey] EDiv
ERand little-endian integer -> [LongTermKey] Rand
```

12. Backed up the existing Linux BlueZ adapter records:

```text
/var/lib/bluetooth-backups/F8:89:D2:83:92:C0-before-m600-key-sync-*
/var/lib/bluetooth-backups/F8:89:D2:83:92:C0-before-m600-extra-identities-*
```

13. Stopped Bluetooth before editing BlueZ state:

```bash
sudo systemctl stop bluetooth
```

14. Added a new BlueZ record for the Windows-current M600 identity:

```text
/var/lib/bluetooth/F8:89:D2:83:92:C0/C6:C0:FD:F1:FB:80/info
```

15. Added fallback BlueZ records for the older Windows M600 identities:

```text
/var/lib/bluetooth/F8:89:D2:83:92:C0/C6:C0:FA:F1:FB:80/info
/var/lib/bluetooth/F8:89:D2:83:92:C0/C6:C0:F8:F1:FB:80/info
```

16. Restarted Bluetooth:

```bash
sudo systemctl start bluetooth
```

17. Verified BlueZ now saw the Windows-current M600 identity as paired and trusted:

```text
Device C6:C0:FD:F1:FB:80
Name: Legion M600 Mouse
Paired: yes
Bonded: yes
Trusted: yes
Blocked: no
```

18. Connected to the Windows-current identity:

```bash
bluetoothctl connect C6:C0:FD:F1:FB:80
```

19. Verified the fix worked:

```text
Connection successful
Connected: yes
Battery Percentage: 95%
```

Example Windows key values:

```text
LTK
IRK
CSRK
ERand
EDIV
Address
AddressType
```

Example BlueZ sections:

```ini
[General]
Name=Legion M600 Mouse
Appearance=0x03c2
AddressType=public
SupportedTechnologies=LE;
Trusted=true
Blocked=false
WakeAllowed=true
Services=00001800-0000-1000-8000-00805f9b34fb;00001801-0000-1000-8000-00805f9b34fb;0000180a-0000-1000-8000-00805f9b34fb;0000180f-0000-1000-8000-00805f9b34fb;00001812-0000-1000-8000-00805f9b34fb;

[IdentityResolvingKey]
Key=<IRK>

[LocalSignatureKey]
Key=<CSRK>
Counter=0
Authenticated=false

[LongTermKey]
Key=<LTK>
Authenticated=0
EncSize=16
EDiv=<EDIV decimal>
Rand=<ERand little-endian decimal>

[ConnectionParameters]
MinInterval=6
MaxInterval=6
Latency=3
Timeout=300
```

For compatibility with BlueZ versions and older naming, it may also be useful to write:

```ini
[PeripheralLongTermKey]
Key=<LTK>
Authenticated=0
EncSize=16
EDiv=<EDIV decimal>
Rand=<ERand little-endian decimal>

[SlaveLongTermKey]
Key=<LTK>
Authenticated=0
EncSize=16
EDiv=<EDIV decimal>
Rand=<ERand little-endian decimal>
```

## Address Variants

The M600 case had multiple Windows-side identities:

```text
C6:C0:FD:F1:FB:80
C6:C0:FA:F1:FB:80
C6:C0:F8:F1:FB:80
```

Linux originally had:

```text
C6:C0:FC:F1:FB:80
```

The newest Windows identity was selected by comparing Windows `LastConnected` timestamps. Syncing the newest identity allowed Linux to connect successfully:

```text
C6:C0:FD:F1:FB:80
Connected: yes
Battery: 95%
```

This means a robust tool should not assume there is only one address per physical device. BLE devices may expose multiple identities or historical records.

## Safety Requirements For A Tool

A tool that automates this should be conservative.

Required behavior:

- Never write to the Windows registry.
- Mount or read Windows hives read-only.
- Detect Windows Fast Startup or unsafe NTFS state when possible.
- Back up `/var/lib/bluetooth` before any write.
- Stop `bluetooth.service` before editing BlueZ records.
- Preserve existing Linux records unless the user explicitly asks to remove them.
- Show a dry-run plan before applying changes.
- Support rollback from backups.
- Clearly show adapter and device addresses before writing.

Suggested commands:

```bash
bt-dualboot-sync scan
bt-dualboot-sync plan --device "Legion M600 Mouse"
sudo bt-dualboot-sync apply --device "Legion M600 Mouse"
sudo bt-dualboot-sync rollback
```

## Matching Strategy

A future implementation should match devices using multiple signals:

- Bluetooth address.
- Device name.
- BLE appearance.
- VID/PID.
- Services, especially HID service `00001812-0000-1000-8000-00805f9b34fb`.
- Windows `LastConnected` timestamp.
- Existing Linux BlueZ records.

When multiple candidates exist, the tool should ask the user to choose rather than guessing silently.

## Risks And Limits

This fix does not prevent Windows from generating new keys later. If the user removes and re-pairs the device in Windows, Linux may need to be synced again.

This also does not solve unrelated problems such as:

- Mouse not being in Bluetooth mode.
- Mouse not advertising.
- Battery or firmware issues.
- Broken Bluetooth firmware.
- BitLocker preventing registry access.
- Windows Fast Startup leaving the NTFS volume unsafe to read.

## Practical Lesson

For dual-boot Bluetooth, the pairing is shared by the peripheral, but the keys are stored separately by each OS. The fix is not to repeatedly re-pair. The fix is to make both operating systems agree on the same Bluetooth bond.
