## Local Development Notes for Ubuntu:

Install Ruby:

```bash
sudo apt-get install ruby-full build-essential zlib1g-dev
```

Setup ruby for local gems

```bash
echo '# Install Ruby Gems to ~/gems' >> ~/.bashrc
echo 'export GEM_HOME="$HOME/gems"' >> ~/.bashrc
echo 'export PATH="$HOME/gems/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

Install bundler

```bash
gem install bundler
```

Clone this repo.  Replace `YOUR-CODE-DIR` with the path to where you keep your projects.

```bash
cd YOUR-CODE-DIR
git clone https://github.com/appcove/egui.info.git
cd egui.info
```

Setup project

```bash
bundle install
```

Run dev server

```bash
./devserver
```



