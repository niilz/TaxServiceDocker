package taxservice;

import javax.enterprise.context.RequestScoped;
import javax.inject.Inject;
import javax.persistence.NoResultException;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.QueryParam;
import javax.ws.rs.core.Application;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;

@Path("/")
@RequestScoped
public class TaxServiceJax extends Application {

	@Inject
	TaxResource taxResource;

	@GET
	@Path("/init")
	@Produces
	public Response init() {
		taxResource.initBaseEntries();
		return Response.ok("Base-Values have been initted").build();
	}

	@GET
	@Path("/get")
	@Produces(MediaType.APPLICATION_JSON)
	public Response getTaxByGoodAndCountry(@QueryParam("good") String good, @QueryParam("country") String countryName) {
		System.out.println("calling getTaxByGoodAndCountry");
		try {
			Tax res = taxResource.findByGoodAndCountry(good, countryName);
			System.out.println("Res from TaxResource: " + res);
			return Response.status(Response.Status.OK).entity(res).build();
		} catch (NoResultException e) {
			return Response.noContent().build();
		}
	}

}
