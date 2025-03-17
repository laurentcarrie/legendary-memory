# <a id="top"/> mdbook / cargo doc

# mdbook

we use [mdbook](https://rust-lang.github.io/mdBook/guide/creating.html) to generate the document you are reading

# cargo doc

we use ``cargo doc`` to generate the documentation of the rust code source

# deploy to github.io

we use github actions to deploy this documentation

# mdbook / cargo doc

it does not seem that these tools were designed to work together, so, in order to have cargo doc generated files
inside the mdbook doc, you will see this file in `.github/workflows/main.yml` action :

      - name: Build Book
        run: |
          cd doc/srs
          mdbook build


      - name: Build cargo doc
        run: |
          ( cd software && cargo doc )
          src=software/target/doc
          target=doc/srs/book
          mkdir -p $target
          cp -R $src/static.files $target/.
          cp -R $src/songbook $target/.
          cp -R software/others $target/.
          find  $target/others -type f  | while read f ; do echo $f ; cp $f $f.txt ; done

basically we :

1. copy the rust doc generated html files in the mdbook generated tree.
2. copy other files we want to see
3. copy some files with the extension `.ext`, so that they can be rendered
