#remove target\data.zip
rm .\target\data.zip

#copy assets/* into assets/assets/* to maintain directory structure in zipfile
mkdir assets/assets

#I know this give a write error but it still seems to run fine
move .\assets\* .\assets\assets\

Add-Type -assembly "system.io.compression.filesystem"
[io.compression.zipfile]::CreateFromDirectory("$pwd\assets", "$pwd\target\data.zip")

#restore directory structure
move .\assets\assets\* .\assets\

rmdir .\assets\assets\