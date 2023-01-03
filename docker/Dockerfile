# escape=`

FROM mcr.microsoft.com/windows/server:ltsc2022

SHELL ["powershell", "-Command", "$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue';"]
WORKDIR C:\

# download sf
ENV sfUrl "https://download.microsoft.com/download/b/8/a/b8a2fb98-0ec1-41e5-be98-9d8b5abf7856/MicrosoftServiceFabric.9.1.1436.9590.exe"

# RUN Invoke-WebRequest https://api.github.com:443

RUN Invoke-WebRequest $env:sfUrl -OutFile $env:Temp/MicrosoftServiceFabric.9.1.1436.9590.exe

RUN Get-ChildItem $env:Temp

RUN & "${env:Temp}\MicrosoftServiceFabric.9.1.1436.9590.exe" /accepteula /sdkcontainerclient

# download sdk
# ENV sfSdkUrl "https://download.microsoft.com/download/D/D/D/DDD408E4-6802-47FB-B0A1-ECF657BEB35F/MicrosoftServiceFabricSDK.3.3.617.msi"

ADD "https://download.microsoft.com/download/D/D/D/DDD408E4-6802-47FB-B0A1-ECF657BEB35F/MicrosoftServiceFabricSDK.3.3.617.msi" C:\TEMP\MicrosoftServiceFabricSDK.msi

RUN msiexec.exe /i "C:\TEMP\MicrosoftServiceFabricSDK.msi" /qn 

# TODO: it seems like the driver still cannot be installed.
# see https://github.com/microsoft/service-fabric-issues/issues/711 for details

