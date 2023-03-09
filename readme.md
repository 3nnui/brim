
# Brim 👒
```
▀█████████▄     ▄████████  ▄█    ▄▄▄▄███▄▄▄▄  
  ███    ███   ███    ███ ███  ▄██▀▀▀███▀▀▀██▄
  ███    ███   ███    ███ ███▌ ███   ███   ███
 ▄███▄▄▄██▀   ▄███▄▄▄▄██▀ ███▌ ███   ███   ███
▀▀███▀▀▀██▄  ▀▀███▀▀▀▀▀   ███▌ ███   ███   ███
  ███    ██▄ ▀███████████ ███  ███   ███   ███
  ███    ███   ███    ███ ███  ███   ███   ███
▄█████████▀    ███    ███ █▀    ▀█   ███   █▀ 
               ███    ███                     
               
A fedora post install tool for some moderate hardening...
```

*This Project is still in very early development... Also this is my first rust project so there will be a lot of refactoring!*


## Threat Level

The principle of hardening applied by this tool is the following:

- minor exploit migration
- forcing user practices
- protecting against rogue USB devices

The result is a stable, almost imperceptible but only moderate hardening of the operating system. For more complete hardening and security, the following projects are recommended:

- [Qubes Os](https://www.qubes-os.org/)
- [kicksecure](https://www.kicksecure.com/)
## Acknowledgements

The Project will probably use the hardened-kernel and hardened-malloc COPR packages currently maintained by samsepi01.

- [HardHatOs Copr](https://copr.fedorainfracloud.org/coprs/samsepi0l/HardHatOS/)

## Sources

The sources for the used hardening principles:

 - [Madaidan linux hardening guide](https://madaidans-insecurities.github.io/guides/linux-hardening.html)
 - [Madaidan Arch hardening script](https://gitlab.com/madaidan/arch-hardening-script)
 - [Original HardHatOs Gtihub](https://github.com/HardHatOS)

## Authors

- [3nnui](https://github.com/3nnui)

## License

[![Hippocratic License HL3-LAW-MIL-SOC-SV-USTA](https://img.shields.io/static/v1?label=Hippocratic%20License&message=HL3-LAW-MIL-SOC-SV-USTA&labelColor=5e2751&color=bc8c3d)](https://firstdonoharm.dev/version/3/0/law-mil-soc-sv-usta.html)