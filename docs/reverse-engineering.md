# Reverse engineering Rightmove


This program will use [Rightmove](https://www.rightmove.co.uk) as the source of properties to search.

## Url query

To find what we need to supply regarding search parameters, let's go to Rightmove and add some values to the search fields to how Rightmove process this.

Search using 'region' as the location identifier, produces the following url;

```
https://www.rightmove.co.uk/property-for-sale/find.html?searchLocation=SW1%2C+South+West+London&useLocationIdentifier=true&locationIdentifier=REGION%5E91989&radius=0.5&minPrice=50000&maxPrice=1000000&minBedrooms=0&maxBedrooms=5&_includeSSTC=on
```

Search using 'postcode' as the location identifier, produces the following url;

```
https://www.rightmove.co.uk/property-for-sale/find.html?searchLocation=SW1A+2AA&useLocationIdentifier=true&locationIdentifier=POSTCODE%5E1246000&radius=0.5&minPrice=50000&maxPrice=1000000&minBedrooms=0&maxBedrooms=5&_includeSSTC=on
```

So we have a base url;

```
https://www.rightmove.co.uk/property-for-sale/find.html?
```

We can then reverse engineer the query parameters in the url in order to create our own `GET` request directly and use this with an http client.
Search parameters we have;

- `searchLocation`
- `useLocationIdentifier`
- `locationIdentifier`
- `radius`
- `minPrice`
- `maxPrice`
- `minBedrooms`
- `maxBedrooms`
- `_includeSSTC`

It looks like even without explicitly selecting the button, the Sold Subject to Contract (SSTC) option seems to be set to on.

## Results parsing

Now that we know how to get results, we need figure out how to process them.
We need to find an identifier for a property that meets our criteria, and we will deal with paginating of results so we can make sure we get them all.

### Property identifier

Using browser to inspect a result property elemet, we see that property result will have (several occurences) an anchor tag with a link to the property;

```
<a data-testid="property-details-lozenge" href="/properties/172182695#/?channel=RES_BUY">
```

We can use this to find parse out all anchor tags with the `data-testid="property-details-lozenge` attribute, and collect uniques `href`s.

### Pagination

If there are lots of results returned, then the results are paginated.
Inspection of the page suggests this defaults to 25 results per page.
Looking a bit deeper in the `html`, we see there is the `&index=` parameter added to the `url` query.
Then inspecting the page dropdown element we can see that this is populated based on the selected value.

```
<select aria-label="Navigate to page" data-testid="paginationSelect" name="Navigate to page">
    <option value="0">1</option>
    <option value="24">2</option>
    <option value="48">3</option>
    <option value="72">4</option>
    <option value="96">5</option>
</select>
```

So, we can grab all the option values from the first page, parse out the option values (avoid hardcoding page=25 results) and then loop over the option values, withget requests for each options value.


### Results count

The results page gives a total results count, see html snippet below.

```html
<div class="ResultsCount_resultsCount__Kqeah">
  <p>
    <span>27</span> results
  </p>
</div>
```

This could be useful for writing a less flaky ingegration test for capturing result hyperlinks.