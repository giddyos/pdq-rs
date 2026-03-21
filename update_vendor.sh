if ! command -v git &> /dev/null
then
    echo "git could not be found. Please install git to use this script."
    exit
fi

if [ ! -d "vendor/pdq/cpp" ]; then
  mkdir -p vendor/pdq/cpp
fi

if [ ! -d "ThreatExchange/.git" ]; then
  echo "Error: ThreatExchange directory does not exist or is not a git repository."
  echo "Please pull the latest changes from the ThreatExchange repository before running this script."
  exit 1
fi

git pull --recurse-submodules

cp -r ThreatExchange/pdq/cpp/common vendor/pdq/cpp/
cp -r ThreatExchange/pdq/cpp/downscaling vendor/pdq/cpp/
cp -r ThreatExchange/pdq/cpp/hashing vendor/pdq/cpp/
echo "Updated vendor/pdq/cpp with latest from ThreatExchange/pdq/cpp"