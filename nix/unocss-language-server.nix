{
  stdenvNoCC,
  fetchFromGitHub,
  nodejs_latest,
  pnpm_9,
}:

stdenvNoCC.mkDerivation (finalAttrs: {
  pname = "unocss-language-server";
  version = "0.1.7";

  src = fetchFromGitHub {
    owner = "xna00";
    repo = finalAttrs.pname;
    tag = "v${finalAttrs.version}";
    hash = "sha256-zvRRK7KvI011cmUpJXsynx0cNDDOMVCVRxsBVeYCMqI=";
  };

  nativeBuildInputs = [
    nodejs_latest
    pnpm_9.configHook
  ];

  pnpmDeps = pnpm_9.fetchDeps {
    inherit (finalAttrs) pname version src;
    fetcherVersion = 2;
    hash = "sha256-CScy3Wn9C4Mh1NQOw3vRXSSji1UqMPUMFA1ghUcOvlo=";
  };

  buildPhase = ''
    runHook preBuild

    pnpm build

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/{bin,lib/${finalAttrs.pname}}
    cp -r . $out/lib/${finalAttrs.pname}
    chmod +x $out/lib/${finalAttrs.pname}/bin/index.js
    ln -s $out/lib/${finalAttrs.pname}/bin/index.js $out/bin/${finalAttrs.pname}

    runHook postInstall
  '';
})
