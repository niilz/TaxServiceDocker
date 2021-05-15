package taxservice;

import javax.enterprise.context.ApplicationScoped;
import javax.persistence.EntityManager;
import javax.persistence.PersistenceContext;
import javax.persistence.TypedQuery;
import javax.transaction.Transactional;

@ApplicationScoped
public class TaxResource {

	@PersistenceContext(unitName = "TaxService")
	EntityManager em;

	@Transactional
	void initBaseEntries() {
		System.out.println(" Initializing Base-Values");
		Country england = new Country("England");
		Country germany = new Country("Deutschland");
		em.persist(england);
		em.persist(germany);
		Tax zigsToEngland = new Tax("Zigaretten", england, 440);
		Tax zigsToGermany = new Tax("Zigaretten", germany, 200);
		em.persist(zigsToEngland);
		em.persist(zigsToGermany);
	}

	Tax findByGoodAndCountry(String good, String countryName) {
		System.out.println("em: " + em);

		TypedQuery<Country> countryQuery = em.createQuery("SELECT c FROM Country c WHERE c.name = :countryName", Country.class);
		System.out.println("query: " + countryQuery);

		countryQuery.setParameter("countryName", countryName);
		System.out.println("created query");

		Country country = countryQuery.getSingleResult();
		System.out.println("country-result: " + country);

		TypedQuery<Tax> taxQuery = em.createQuery("SELECT t FROM Tax t WHERE t.destination = :country AND t.good = :good", Tax.class);
		taxQuery.setParameter("country", country);
		taxQuery.setParameter("good", good);

		Tax res = taxQuery.getSingleResult();
		System.out.println("tax-query-result: " + res);

		return res;
	}

	void insertNewTaxForGoodAndCountry(String good, String countryName, int amount) {
		Country country = new Country(countryName);
		Tax tax = new Tax(good, country, amount);
	}
}
