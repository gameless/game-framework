rm .\target\game-framework.zip

#build project
cargo build --release

#build assets
.\package_assets.ps1

#create folder to put everything in before zipping
mkdir .\target\package\

#copy everything into target\package\
cp .\target\data.zip .\target\package\data.zip
cp .\target\release\framework.exe .\target\package\framework.exe

#zip up target\package\
Add-Type -assembly "system.io.compression.filesystem"
[io.compression.zipfile]::CreateFromDirectory("$pwd\target\package\", "$pwd\target\game-framework.zip")
